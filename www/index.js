import {
    init,
    loadCode,
    saveCode,
    saveCodeHist,
    loadCodeHist
} from './common.js';
import { Editor, Highlighter } from 'pinguim-editor';
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
actualBtn.addEventListener('change', function () {
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


const editor = new Editor({
    targetTextArea: document.getElementById('userinput'),
    targetPre: document.getElementById('codeediting'),
    currLineSpan: document.getElementById('input-line'),
    currColumnSpan: document.getElementById('input-column'),
    saveCode,
    loadCode,
    saveCodeHist,
    loadCodeHist,
    highlighter: new Highlighter(
        { className: 'comment', regex: /--.*\n/ },
        { className: 'reserved', regex: /\blet\b|\bin\b/ },
        { className: 'number', regex: /\b[0-9]+\b/ },
        { className: 'punctuation', regex: /\\|\.|=|;/ },
        {
            className: 'punctuation',
            bracket: { name: 'parens', direction: 'opening' },
            regex: /\(/,
        },
        {
            className: 'punctuation',
            bracket: { name: 'parens', direction: 'closing' },
            regex: /\)/,
        },
    ),
});

// Init
init(() => {
    editor.load();
});
