// Adds a copy button after each <code class="copy"> in the docs, which works like the site's CopyButton
// (src/components/copy_button.rs): it copies the code's text, and shows a check for 2 seconds. Write it in the Markdown
// as <code class="copy">pvcs update</code>. Without JavaScript, it's plain inline code.
(function copyButtons() {
    // Phosphor's copy and check icons, as in src/icons/ui.rs
    const COPY = '<svg viewBox="0 0 256 256" aria-hidden="true"><path d="M216 32H88a8 8 0 0 0-8 8v40H40a8 8 0 0 0-8 8v128a8 8 0 0 0 8 8h128a8 8 0 0 0 8-8v-40h40a8 8 0 0 0 8-8V40a8 8 0 0 0-8-8m-56 176H48V96h112Zm48-48h-32V88a8 8 0 0 0-8-8H96V48h112Z"/></svg>';
    const CHECK = '<svg viewBox="0 0 256 256" aria-hidden="true"><path d="m229.66 77.66l-128 128a8 8 0 0 1-11.32 0l-56-56a8 8 0 0 1 11.32-11.32L96 188.69L218.34 66.34a8 8 0 0 1 11.32 11.32"/></svg>';

    for (const code of document.querySelectorAll('main code.copy')) {
        const text = code.textContent;
        const button = document.createElement('button');
        button.type = 'button';
        button.className = 'copy-button';
        button.title = 'Copy ' + text;
        button.setAttribute('aria-label', button.title);
        button.innerHTML = COPY;
        // Tells screen readers it worked, as the icon can't.
        const status = document.createElement('span');
        status.className = 'sr-only';
        status.setAttribute('aria-live', 'polite');

        let reset;
        button.addEventListener('click', function() {
            // Safari only lets a page write to the clipboard while it's handling a click, so start the write here.
            navigator.clipboard.writeText(text).then(function() {
                button.innerHTML = CHECK;
                status.textContent = 'Copied';
                clearTimeout(reset);
                reset = setTimeout(function() {
                    button.innerHTML = COPY;
                    status.textContent = '';
                }, 2000);
            }, function() { });
        });

        code.after(button, status);
    }
})();
