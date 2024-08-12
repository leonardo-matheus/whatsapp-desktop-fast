document.addEventListener('DOMContentLoaded', (event) => {
    function removeAd() {
        const adElement = document.querySelector('._al_d','x1c4vz4f xs83m0k xdl72j9 x1g77sc7 xozqiw3 x1oa3qoh x12fk4p8 xeuugli x2lwn1j xaw8158 x1q0g3np x6s0dn4 x182zwpg x3nfvp2');
        if (adElement) {
            adElement.remove();
        }
    }

    const observer = new MutationObserver(() => {
        removeAd();
    });

    observer.observe(document.body, {
        childList: true,
        subtree: true,
        attributes: true
    });

    // Chamada inicial caso o elemento já esteja presente
    removeAd();
});
