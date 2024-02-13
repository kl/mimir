(function () {

    let tokenOld = "";

    async function reload () {

        let tokenNew = "";

        try {
            let response = await fetch("/health_check");

            if (response.status === 200) {
                tokenNew = await response.text();
            }
        } catch (_error) {
            console.error(_error);
        }

        if (tokenOld !== "" && tokenNew !== "" && tokenNew !== tokenOld) {
            if (window.history.scrollRestoration !== undefined) {
                window.history.scrollRestoration = "auto";
            }
            window.history.go();
        } else {
            window.setTimeout(reload, 1000);
        }
        if (tokenNew !== "") {
            tokenOld = tokenNew;
        }
    }
    window.setTimeout(reload, 1000);
})();
