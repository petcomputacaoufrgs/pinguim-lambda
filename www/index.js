import {
    init,
    getStorage,
    setStorage,
} from './common.js';
import {
    makeVariable,
    makeApplication,
    makeLambda,
    stringify,
} from './lambda.js';
import {
    drawTerm,
    initSvgRoot
} from './draw.js';
import * as wasm from "lambda-wasm";

const actualBtn = document.getElementById('upload_button');
const fileChosen = document.getElementById('file-chosen');
actualBtn.addEventListener('change', function(){
    fileChosen.textContent = this.files[0].name
});


init(() => {
    const svgTarget = document.getElementById('tree');
    initSvgRoot(svgTarget);
    const term = makeLambda(
        'f',
        makeLambda(
            'x',
            makeApplication(
                makeApplication(
                    makeLambda(
                        'f',
                        makeLambda(
                            'x',
                            makeApplication(
                                makeVariable('f'),
                                makeApplication(
                                    makeVariable('f'),
                                    makeVariable('x')
                                ),
                            ),
                        ),
                    ),
                    makeVariable('f'),
                ),
                makeApplication(
                    makeApplication(
                        makeLambda(
                            'f',
                            makeLambda(
                                'x',
                                makeApplication(
                                    makeVariable('f'),
                                    makeApplication(
                                        makeVariable('f'),
                                        makeApplication(
                                            makeVariable('f'),
                                            makeVariable('x')
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        makeVariable('f'),
                    ),
                    makeVariable('x'),
                ),
            ),
        ),
    );
    drawTerm(term, svgTarget);
});

// Highlight 
const textAreaHTML = document.getElementById('userinput');
const codeAreaHTML = document.getElementById('codeholder');
const preAreaHTML = document.getElementById('codeediting');

class Highlighter {
    constructor(...types) {
        this.types = types;

        const alternatives = types.map(type => '(' + type.regex.source + ')');
        const flags = types.reduce((flags, type) => {
            for (const flag of type.regex.flags) {
                if (flags.indexOf(flag) < 0) {
                    flags += flag;
                }
            }
            return flags;
        }, '');

        this.splitRegex = new RegExp(alternatives.join('|'), flags);
    }

    highlight(baseText, targetElement) {
        targetElement.innerHTML = '';

        for (let piece of baseText.split(this.splitRegex)) {
            piece = piece || "";

            const type = this.types.find(type => type.regex.test(piece));

            let child;
            if (type !== undefined) {
                child = document.createElement('span');
                child.setAttribute('class', type.className);
                child.textContent = piece;
            } else {
                child = document.createTextNode(piece);
            }
            targetElement.appendChild(child);
        }
    }
}

const highlighter = new Highlighter(
    { className: 'reserved', regex: /\blet\b|\bin\b/ },
    { className: 'punctuation', regex: /\\|\.|=|;|\(|\)/ },
    { className: 'number', regex: /\b[0-9]+\b/ },
);

const highlight = () => {
    const baseText = textAreaHTML.value;
    highlighter.highlight(baseText, codeAreaHTML);
    setStorage(baseText);
};

const handleKeys = {
    'Tab': (e) => handleTab(e),
    'Enter': (e) => handleEnter(e),
    'Backspace': (e) => handleBackspace(e),
    '(': (e) => handleBracket(e),
    '{': (e) => handleCurly(e)
};

textAreaHTML.addEventListener('keyup', (evt) => highlight());

textAreaHTML.addEventListener('keydown', (e) => {
     try { handleKeys[e.key](e) }
     catch(e) {}
});

textAreaHTML.addEventListener('scroll', (e) => handleScroll());

const handleScroll = () => {
    preAreaHTML.scrollTop = textAreaHTML.scrollTop;
    preAreaHTML.scrollLeft = textAreaHTML.scrollLeft;
}

const handleTab = (e) => {
    e.preventDefault();
    const start = textAreaHTML.selectionStart;
    const end = textAreaHTML.selectionEnd;

    textAreaHTML.value = textAreaHTML.value.substring(0, start) + 
        `    ` + textAreaHTML.value.substring(end);

    textAreaHTML.selectionStart = textAreaHTML.selectionEnd = start + 4;
};

const handleEnter = (e) => {
    const start = textAreaHTML.selectionStart;
    const end = textAreaHTML.selectionEnd;

    if((textAreaHTML.value[textAreaHTML.selectionStart - 1] == '{') && 
        (textAreaHTML.value[textAreaHTML.selectionStart] == '}')) {
        e.preventDefault();
        const start = textAreaHTML.selectionStart;
        const end = textAreaHTML.selectionEnd;

        textAreaHTML.value = textAreaHTML.value.substring(0, start) +
            "\n" + `    ` + "\n" + textAreaHTML.value.substring(end);

        textAreaHTML.selectionStart = textAreaHTML.selectionEnd = start + 5;
    }
};

const handleBackspace = (e) => {
    const start = textAreaHTML.selectionStart;
    const end = textAreaHTML.selectionEnd;

    if(((textAreaHTML.value[textAreaHTML.selectionStart - 1] == '(') && 
        (textAreaHTML.value[textAreaHTML.selectionStart] == ')')) 
        || 
        ((textAreaHTML.value[textAreaHTML.selectionStart - 1] == '{') && 
        (textAreaHTML.value[textAreaHTML.selectionStart] == '}'))) {
            
        e.preventDefault();

        textAreaHTML.value = textAreaHTML.value.substring(0, start).slice(0, start - 1)
            + textAreaHTML.value.substring(end).slice(1, end);

        textAreaHTML.selectionStart = textAreaHTML.selectionEnd = start - 1;
    }
};

const handleBracket = (e) => {
    e.preventDefault();
    const start = textAreaHTML.selectionStart;
    const end = textAreaHTML.selectionEnd;

    textAreaHTML.value = textAreaHTML.value.substring(0, start) + 
        "()" + textAreaHTML.value.substring(end);

    textAreaHTML.selectionStart = textAreaHTML.selectionEnd = end + 1;
};

const handleCurly = (e) => {
    e.preventDefault();
    const start = textAreaHTML.selectionStart;
    const end = textAreaHTML.selectionEnd;

    textAreaHTML.value = textAreaHTML.value.substring(0, start) + 
        "{}" + textAreaHTML.value.substring(end);

    textAreaHTML.selectionStart = textAreaHTML.selectionEnd = end + 1;
};

// Local Storage
const getLastCode = () => {
    textAreaHTML.value = getStorage();
    highlight();
};

// Init
init(() => {
    getLastCode();
});
