#![forbid(unsafe_code)]
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use askama::Template;
use axum::{
    extract::ConnectInfo,
    response::Html,
    routing::{get, post},
    Extension, Router,
};
use memory_serve::{load_assets, MemoryServe};
use minify_html::{minify, Cfg};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::{postgres::PgPoolOptions, types::ipnetwork::IpNetwork};
use std::net::SocketAddr;
use std::time::SystemTime;

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(400)
        .connect("<YOUR DB CONNECTION STRING>")
        .await
        .unwrap();

    let asset_router = MemoryServe::new(load_assets!("templates/assets"))
        .cache_control(memory_serve::CacheControl::Medium)
        .into_router();

    let app = Router::new()
        .route("/", get(hlavni_stranka))
        .route("/hodnoceni", get(hodnoceni))
        .route("/jiz-hodnoceno", get(jiz_hodnoceno))
        .route("/podekovani", get(podekovani))
        .route("/informace", get(informace))
        .route("/predbeznevysledky", get(predbeznevysledky))
        .route("/predbeznevysledky/data", get(predbeznevysledkyhx))
        .route("/hledatucitele", post(hledatucitele))
        .route("/zapsat", post(zapsat))
        .route("/zpravy", get(zpravy))
        .route("/zpravy/poslat", get(zpravyposlat))
        .route("/zpravy/zapsat", post(zpravyzapsat))
        .route("/zpravy/hx", get(zpravyhx))
        .route("/gdpr", get(gdpr))
        .route("/vysledky", get(vysledky))
        .route("/technickevysledky", get(technicke_vysledky))
        .merge(asset_router)
        .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    println!("Poslouchám na: 0.0.0.0:80");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

fn minifi_html(html: String) -> Vec<u8> {
    let mut cfg = Cfg::new();
    cfg.keep_comments = false;
    cfg.minify_css = true;
    cfg.minify_js = true;
    let result: Vec<u8> = minify(html.as_bytes(), &cfg);
    result
}

#[derive(Template)]
#[template(path = "pages/hlavnistranka.html", escape = "none")]
struct HlavniStrankaTemplate {}

async fn hlavni_stranka() -> axum::response::Html<Vec<u8>> {
    let hello = HlavniStrankaTemplate {};
    Html(minifi_html(hello.render().unwrap()))
}

#[derive(Template)]
#[template(path = "pages/hodnoceni.html", escape = "none")]
struct HodnoceniTemplate {}

async fn hodnoceni() -> axum::response::Html<Vec<u8>> {
    let hello = HodnoceniTemplate {};
    Html(minifi_html(hello.render().unwrap()))
}

#[derive(Serialize, Deserialize)]
struct HledatUciteleInput {
    ucitel: String,
}

async fn hledatucitele(
    Extension(pool): Extension<PgPool>,
    hledany_text: axum::extract::Form<HledatUciteleInput>,
) -> axum::response::Html<String> {
    let hledany_text = format!("%{}%", hledany_text.ucitel);

    let seznam_ucitelu = match sqlx::query!(
        "SELECT jmeno FROM ucitele WHERE jmeno ILIKE $1 LIMIT 10",
        hledany_text
    )
    .fetch_all(&pool)
    .await
    {
        Ok(data) => data,
        Err(_) => return Html("<div>Nepodařilo se najít nikoho takového</div>".to_string()),
    };

    let vysledek: Vec<String> = seznam_ucitelu
        .par_iter()
        .map(|ucitel| {
            format!(
                "<li class=\"modal-item vybratelnyucitel\">{}</li>",
                ucitel.jmeno
            )
        })
        .collect();

    let html_response = vysledek.join("");

    Html(html_response)
}

#[derive(Serialize, Deserialize)]
struct ZapsatInput {
    obor: String,
    teacher_good_1: Option<String>,
    teacher_good_2: Option<String>,
    teacher_good_3: Option<String>,
    teacher_good_4: Option<String>,
    teacher_good_5: Option<String>,
    teacher_bad_1: Option<String>,
    teacher_bad_2: Option<String>,
    teacher_bad_3: Option<String>,
    teacher_bad_4: Option<String>,
    teacher_bad_5: Option<String>,
}
#[derive(Serialize, Deserialize)]
struct HlasStruct {
    ucitel: String,
    pozitivni: bool,
}
#[derive(Serialize, Deserialize)]
struct HlasovaniInfoStruct {
    ipadresa: IpNetwork,
    useragent: String,
    obor: String,
    cas: i64,
}
async fn zapsat(
    ConnectInfo(ipadresa): ConnectInfo<SocketAddr>,
    Extension(pool): Extension<PgPool>,
    headers: axum::http::HeaderMap,
    vstup: axum::extract::Form<ZapsatInput>,
) -> axum::response::Html<String> {
    let mut hlasy = Vec::new();

    if let Some(ucitel) = vstup.teacher_good_1.clone().filter(|s| !s.is_empty()) {
        hlasy.push(HlasStruct {
            ucitel,
            pozitivni: true,
        });
    }
    if let Some(ucitel) = vstup.teacher_good_2.clone().filter(|s| !s.is_empty()) {
        hlasy.push(HlasStruct {
            ucitel,
            pozitivni: true,
        });
    }
    if let Some(ucitel) = vstup.teacher_good_3.clone().filter(|s| !s.is_empty()) {
        hlasy.push(HlasStruct {
            ucitel,
            pozitivni: true,
        });
    }
    if let Some(ucitel) = vstup.teacher_good_4.clone().filter(|s| !s.is_empty()) {
        hlasy.push(HlasStruct {
            ucitel,
            pozitivni: true,
        });
    }
    if let Some(ucitel) = vstup.teacher_good_5.clone().filter(|s| !s.is_empty()) {
        hlasy.push(HlasStruct {
            ucitel,
            pozitivni: true,
        });
    }

    if let Some(ucitel) = vstup.teacher_bad_1.clone().filter(|s| !s.is_empty()) {
        hlasy.push(HlasStruct {
            ucitel,
            pozitivni: false,
        });
    }
    if let Some(ucitel) = vstup.teacher_bad_2.clone().filter(|s| !s.is_empty()) {
        hlasy.push(HlasStruct {
            ucitel,
            pozitivni: false,
        });
    }
    if let Some(ucitel) = vstup.teacher_bad_3.clone().filter(|s| !s.is_empty()) {
        hlasy.push(HlasStruct {
            ucitel,
            pozitivni: false,
        });
    }
    if let Some(ucitel) = vstup.teacher_bad_4.clone().filter(|s| !s.is_empty()) {
        hlasy.push(HlasStruct {
            ucitel,
            pozitivni: false,
        });
    }
    if let Some(ucitel) = vstup.teacher_bad_5.clone().filter(|s| !s.is_empty()) {
        hlasy.push(HlasStruct {
            ucitel,
            pozitivni: false,
        });
    }

    let useragent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("undefined")
        .to_string();
    let hlasovani = HlasovaniInfoStruct {
        ipadresa: ipadresa.ip().into(),
        useragent,
        obor: vstup.obor.clone(),
        cas: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .try_into()
            .unwrap(),
    };
    tokio::task::spawn(zapsatdodb(pool, hlasovani, hlasy));
    Html("kontrola dat OK".to_string() + "<script>window.location.replace('/podekovani');</script>")
}

async fn zapsatdodb(pool: PgPool, hlasovani: HlasovaniInfoStruct, hlasy: Vec<HlasStruct>) {
    let idhlasovani = sqlx::query!(
        "INSERT INTO hlasovani (ipadresa,useragent,obor,cas) VALUES ($1,$2,$3,$4) RETURNING id;",
        hlasovani.ipadresa,
        hlasovani.useragent,
        hlasovani.obor,
        hlasovani.cas
    )
    .fetch_one(&pool)
    .await
    .unwrap()
    .id;
    for hlas in hlasy {
        let _ = sqlx::query!(
            "INSERT INTO hlasy (ucitel,hlasovani,pozitivni) VALUES ($1,$2,$3);",
            hlas.ucitel,
            idhlasovani,
            hlas.pozitivni
        )
        .execute(&pool)
        .await;
    }
}

#[derive(Template)]
#[template(path = "pages/predbeznevysledky.html", escape = "none")]
struct PredbezneVysledkyTemplate {}

async fn predbeznevysledky() -> axum::response::Html<Vec<u8>> {
    let hello = PredbezneVysledkyTemplate {};
    Html(minifi_html(hello.render().unwrap()))
}

#[derive(Template)]
#[template(path = "pages/predbeznevysledky-hx.html", escape = "none")]
struct PredbezneVysledkyHXTemplate {
    ucitelejson: String,
    pocethlasujson: String,
}

async fn predbeznevysledkyhx(Extension(pool): Extension<PgPool>) -> axum::response::Html<Vec<u8>> {
    let predbeznevysledky = sqlx::query!(
        "SELECT 
        u.jmeno, u.uuid, u.gdpr,
        COUNT(DISTINCT h.hlasovani) AS positive_votes
    FROM 
        public.ucitele u
    JOIN 
        public.hlasy h ON u.jmeno = h.ucitel
    WHERE 
        h.pozitivni = true
    GROUP BY 
        u.jmeno
    ORDER BY
    positive_votes ASC;"
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    let mut ucitele: Vec<String> = Vec::new();
    let mut pocethlasu: Vec<i64> = Vec::new();
    for polozka in predbeznevysledky {
        if polozka.gdpr {
            ucitele.push(polozka.jmeno);
        } else {
            ucitele.push(polozka.uuid.to_string());
        }
        pocethlasu.push(polozka.positive_votes.unwrap());
    }
    let ucitelejson = sonic_rs::to_string(&ucitele).unwrap();
    let pocethlasujson = sonic_rs::to_string(&pocethlasu).unwrap();

    let hello = PredbezneVysledkyHXTemplate {
        ucitelejson,
        pocethlasujson,
    };
    Html(minifi_html(hello.render().unwrap()))
}

#[derive(Template)]
#[template(path = "pages/jiz-hodnoceno.html", escape = "none")]
struct JizHodnocenoTemplate {}

async fn jiz_hodnoceno() -> axum::response::Html<Vec<u8>> {
    let hello = JizHodnocenoTemplate {};
    Html(minifi_html(hello.render().unwrap()))
}

#[derive(Template)]
#[template(path = "pages/podekovani.html", escape = "none")]
struct PodekovaniTemplate {}

async fn podekovani() -> axum::response::Html<Vec<u8>> {
    let hello = PodekovaniTemplate {};
    Html(minifi_html(hello.render().unwrap()))
}

#[derive(Template)]
#[template(path = "pages/informace.html", escape = "none")]
struct InformaceTemplate {}

async fn informace() -> axum::response::Html<Vec<u8>> {
    let hello = InformaceTemplate {};
    Html(minifi_html(hello.render().unwrap()))
}

#[derive(Template)]
#[template(path = "pages/zpravy.html", escape = "none")]
struct ZpravyTemplate {}

async fn zpravy() -> axum::response::Html<Vec<u8>> {
    let hello = ZpravyTemplate {};
    Html(minifi_html(hello.render().unwrap()))
}

#[derive(Template)]
#[template(path = "pages/zpravy-poslat.html", escape = "none")]
struct ZpravyPoslatTemplate {}

async fn zpravyposlat() -> axum::response::Html<Vec<u8>> {
    let hello = ZpravyPoslatTemplate {};
    Html(minifi_html(hello.render().unwrap()))
}

#[derive(Template)]
#[template(path = "pages/zpravy-hx.html")]
struct ZpravyHXTemplate {
    adresat: String,
    zprava: String,
    odpoved: String,
    jeodpoved: bool,
}
async fn zpravyhx(Extension(pool): Extension<PgPool>) -> axum::response::Html<String> {
    let seznamzprav = sqlx::query!("SELECT ucitel, text, odpoved FROM zpravy ORDER BY cas DESC")
        .fetch_all(&pool)
        .await
        .unwrap();
    let renderings: Result<Vec<_>, _> = seznamzprav
        .par_iter()
        .map(|zprava| {
            let jeodpoved;
            if zprava.odpoved.is_some() {
                jeodpoved = true;
            } else {
                jeodpoved = false;
            }
            let zpravatext = ZpravyHXTemplate {
                adresat: zprava.ucitel.clone(),
                zprava: zprava.text.clone(),
                odpoved: zprava.odpoved.clone().unwrap_or_default(),
                jeodpoved,
            };
            zpravatext.render()
        })
        .collect();
    let vysledek = renderings.unwrap().join("");
    Html(vysledek)
}

#[derive(Serialize, Deserialize)]
struct ZpravaForm {
    teacher_name: String,
    zprava: String,
}
#[derive(Serialize, Deserialize)]
struct ZpravaStruct {
    ipadresa: IpNetwork,
    useragent: String,
    ucitel: String,
    zprava: String,
    cas: i64,
}
async fn zpravyzapsat(
    ConnectInfo(ipadresa): ConnectInfo<SocketAddr>,
    Extension(pool): Extension<PgPool>,
    headers: axum::http::HeaderMap,
    vstup: axum::extract::Form<ZpravaForm>,
) -> axum::response::Html<String> {
    let useragent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("undefined")
        .to_string();
    let zprava = ZpravaStruct {
        ipadresa: ipadresa.ip().into(),
        useragent,
        ucitel: vstup.teacher_name.clone(),
        zprava: vstup.zprava.clone(),
        cas: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .try_into()
            .unwrap(),
    };
    tokio::task::spawn(zapsatzpravu(pool, zprava));
    Html("kontrola dat OK".to_string() + "<script>window.location.replace('/zpravy');</script>")
}

async fn zapsatzpravu(pool: PgPool, zprava: ZpravaStruct) {
    let _ = sqlx::query!(
        "INSERT INTO zpravy (ipadresa,useragent,ucitel,text,cas) VALUES ($1,$2,$3,$4,$5)",
        zprava.ipadresa,
        zprava.useragent,
        zprava.ucitel,
        zprava.zprava,
        zprava.cas
    )
    .execute(&pool)
    .await;
}

#[derive(Template)]
#[template(path = "pages/gdpr.html", escape = "none")]
struct GDPRTemplate {}

async fn gdpr() -> axum::response::Html<Vec<u8>> {
    let hello = GDPRTemplate {};
    Html(minifi_html(hello.render().unwrap()))
}


#[derive(Template)]
#[template(path = "pages/vysledky.html", escape = "none")]
struct VysledkyTemplate {}

async fn vysledky() -> axum::response::Html<Vec<u8>> {
    let hello = VysledkyTemplate {};
    Html(minifi_html(hello.render().unwrap()))
}

#[derive(Template)]
#[template(path = "pages/technickevysledky.html", escape = "none")]
struct TechnickeVysledkyTemplate {}

async fn technicke_vysledky() -> axum::response::Html<Vec<u8>> {
    let hello = TechnickeVysledkyTemplate {};
    Html(minifi_html(hello.render().unwrap()))
}
