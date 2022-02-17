import * as styles from './styles.css';
import * as commonStyles from './common_styles.css';

// Código para verificar se o wasm é suportado]
// Retirado de https://www.syncfusion.com/faq/how-can-i-check-if-a-browser-supports-webassembly
const supported = (() => {
    try {
        if (typeof WebAssembly === 'object'
            && typeof WebAssembly.instantiate === 'function') {
            const module = new WebAssembly.Module(Uint8Array.of(0x0, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00));
            if (module instanceof WebAssembly.Module)
                return new WebAssembly.Instance(module) instanceof WebAssembly.Instance;
        }
    } catch (e) { }
    return false;
})();

if (!supported) {
    alert("Seu navegador não suporta WebAssembly\nmude de navegador ou use a versão antiga");
}

const toggleSwitch = document.querySelector('.theme-switch input[type="checkbox"]');
const toggleIcon = document.getElementById('toggle-icon');
function switchTheme(e) {
    if (e.target.checked) {
        document.documentElement.setAttribute('data-theme', 'dark');
        toggleIcon.innerHTML = 'dark_mode';
    }
    else {
        document.documentElement.setAttribute('data-theme', 'light');
        toggleIcon.innerHTML = 'light_mode';
    }
}
toggleSwitch.addEventListener('change', switchTheme, false);

const currentTheme = localStorage.getItem('theme') ? localStorage.getItem('theme') : null;
if (currentTheme) {
    document.documentElement.setAttribute('data-theme', currentTheme);

    if (currentTheme === 'dark') {
        toggleSwitch.checked = true;
    }
}

export const init = (() => {
    let handlers = [];

    function callAllHandlers() {
        const oldHandlers = handlers;
        handlers = [];
        for (const handler of oldHandlers) {
            handler();
        }
    }

    window.addEventListener('load', () => {
        callAllHandlers();
    });

    window.addEventListener('DOMContentLoaded', () => {
        callAllHandlers();
    });

    return (handler) => {
        handlers.push(handler);

        if (document.readyState == 'complete') {
            callAllHandlers();
        }
    };
})();

const storagePrefix = "pinguim.lambda";
const storageCodeKey = storagePrefix + '.userCode';
const storageCodeHistKey = storagePrefix + '.userCodeHistory';

// Local Storage
export const saveCode = baseText => {
    localStorage.setItem(storageCodeKey, baseText);
};

export const loadCode = () => {
    return localStorage.getItem(storageCodeKey);
};

export const saveCodeHist = array => {
    localStorage.setItem(storageCodeHistKey, JSON.stringify(array));
};

export const loadCodeHist = () => {
    return JSON.parse(localStorage.getItem(storageCodeHistKey));
};
