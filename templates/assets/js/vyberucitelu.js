// Počkejte, až se DOM plně načte, než spustíte skript
document.addEventListener('DOMContentLoaded', () => {
    // Vyberte všechny vstupní elementy s třídou 'item-input'
    const inputs = document.querySelectorAll('.item-input');
    // Inicializujte novou instanci modálního okna Bootstrap pro element s ID 'itemPopup'
    const modal = new bootstrap.Modal(document.getElementById('itemPopup'));

    // Definujte funkci pro zobrazení modálního okna a zvýraznění aktivního vstupu
    function showModal(input) {
        modal.show(); // Zobrazit modální okno
        inputs.forEach(inp => inp.classList.remove('active')); // Odebrat třídu 'active' ze všech vstupů
        input.classList.add('active'); // Přidat třídu 'active' k aktuálně kliknutému vstupu
    }

    // Přidejte posluchače události kliknutí ke každému vstupu, který spustí funkci showModal
    inputs.forEach(input => {
        input.addEventListener('click', function () {
            showModal(this); // 'this' odkazuje na kliknutý vstupní element
        });
    });

    // Vyberte element '.modal-body' uvnitř modálního okna 'itemPopup'
    const modalBody = document.getElementById('itemPopup').querySelector('.modal-body');
    // Přidejte posluchače události kliknutí k tělu modálního okna
    modalBody.addEventListener('click', function (event) {
        // Zkontrolujte, zda je cíl události kliknutí element s třídou 'modal-item'
        if (event.target && event.target.matches('.modal-item')) {
            // Vyberte aktivní pole pro zobrazení vstupu
            const activeInputDisplay = document.querySelector('.item-input.active');
            // Najděte odpovídající skrytý vstup úpravou ID aktivního vstupu
            const activeInputHidden = document.getElementById(activeInputDisplay.id.replace('_display', ''));
            // Pokud jsou oba prvky nalezeny, aktualizujte jejich hodnoty na obsah kliknuté položky
            if (activeInputDisplay && activeInputHidden) {
                activeInputDisplay.value = event.target.textContent; // Aktualizovat viditelný vstup
                activeInputHidden.value = event.target.textContent; // Aktualizovat skrytý vstup
                modal.hide(); // Skrýt modální okno po výběru
            }
        }
    });
});