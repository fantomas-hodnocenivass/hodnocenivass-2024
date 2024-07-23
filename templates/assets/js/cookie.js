if (zkontrolovatSusenku()) 
{
    var btn = document.getElementById("zacit_hodnotit")
    
    if(btn)
    {
        btn.href = "/jiz-hodnoceno";
    } 

    if (window.location.pathname === "/hodnoceni")
    {
        window.location.replace("/jiz-hodnoceno");
    }  
}

function obnovitStranku() {
    location.reload(true);
}

// Zjistíme jestli se uživatel nevrátil na stránku pomocí šipky zpět
window.addEventListener('pageshow', function(event) {
    var historyTraversal = event.persisted || 
                           (typeof window.performance != 'undefined' && 
                            window.performance.navigation.type === 2);
    if (historyTraversal && window.location.pathname === "/hodnoceni") {
        // Obnovíme ji
        obnovitStranku();
    }
});

function zkontrolovatSusenku() {
    // Split the cookie string into an array of key-value pairs
    var cookies = document.cookie.split(';');

    // Loop through each cookie to find the one named "hlasoval"
    for (var i = 0; i < cookies.length; i++) {
        var cookie = cookies[i].trim();

        // Zjistit jestli susenka zacina na "hlasoval="
        if (cookie.indexOf("hlasoval=") === 0) {
            // Ziskat hodnotu susenky
            var cookieValue = cookie.substring("hlasoval=".length, cookie.length);
            // Zjistit jestli hodnota je "true"
            if (cookieValue === "true") {
                // Susenka "hlasoval" je rovna true
                return true;
            }
        }
    }

    // Susenka "hlasoval" neni urcena, nebo neni true
    return false;
}

function pridatSusenku()
{
    document.cookie = "hlasoval=true; path=/; expires=Sat, 11 Sep 2024 14:14:14 UTC;";
}