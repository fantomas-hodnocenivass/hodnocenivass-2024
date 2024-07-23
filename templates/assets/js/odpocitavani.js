// Nastavení cílového data pro odpočítávání
var countDownDate = new Date("2024-04-08T00:00:00+02:00").getTime();

// Vytvoření intervalu, který se opakuje každou sekundu
var x = setInterval(function () {
  // Získání aktuálního času
  var now = new Date().getTime();
  
  // Výpočet zbývajícího času do cílového data
  var distance = countDownDate - now;
  
  // Převod zbývajícího času na dny, hodiny, minuty a sekundy
  var days = Math.floor(distance / (1000 * 60 * 60 * 24));
  var hours = Math.floor((distance % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
  var minutes = Math.floor((distance % (1000 * 60 * 60)) / (1000 * 60));
  var seconds = Math.floor((distance % (1000 * 60)) / 1000);
  
  // Zobrazení zbývajícího času v elementu s ID 'odpocitavani'
  document.getElementById("odpocitavani").innerHTML = days + "d " + hours + "h " + minutes + "m " + seconds + "s ";
  
  // Pokud je čas vypršel, zastavení intervalu a zobrazení zprávy
  if (distance < 0) {
    clearInterval(x);
    document.getElementById("odpocitavani").innerHTML = "EXPIRED";
  }
}, 1000);