import * as styles from './styles.css';
import * as commonStyles from './common_styles.css';

const toggleSwitch = document.querySelector('.theme-switch input[type="checkbox"]');
function switchTheme(e) {
    if (e.target.checked) {
        document.documentElement.setAttribute('data-theme', 'dark');
    }
    else {
        document.documentElement.setAttribute('data-theme', 'light');
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

const actualBtn = document.getElementById('upload_button');
const fileChosen = document.getElementById('file-chosen');
actualBtn.addEventListener('change', function(){
    fileChosen.textContent = this.files[0].name
});

export const init = (() => {
    let handlers = [];

    function callAllHandlers() {
        const oldHandlers = handlers;
        handlers = [];
        for (const handler of oldHandlers) {
            handler();
        }
    }

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

// Código para verificar se o wasm é suportado]
// Retirado de https://www.syncfusion.com/faq/how-can-i-check-if-a-browser-supports-webassembly
const supported = (() => {
    try {
        if (typeof WebAssembly === 'object'
            && typeof WebAssembly.instantiate === 'function')
        {
            const module = new WebAssembly.Module(Uint8Array.of(0x0, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00));
            if (module instanceof WebAssembly.Module)
                return new WebAssembly.Instance(module) instanceof WebAssembly.Instance;
        }
    } catch (e) { }
    return false;
})();

export function throwNonLambda() {
    throw 'Expected a variable, application or lambda'
}

export function makeVariable(name) {
    return { varname: name };
}

export function isVariable(term) {
    return 'varname' in term;
}

export function makeApplication(func, argument) {
    return { function: func, argument: argument };
}

export function isApplication(term) {
    return 'function' in term && 'argument' in term;
}

export function makeLambda(parameter, body) {
    return { parameter: parameter, body: body };
}

export function isLambda(term) {
    return 'parameter' in term && 'body' in term;
}

export function stringifyTerm(term) {
    if (isVariable(term)) {
        return term.varname;
    }

    if (isApplication(term)) {
        let func = stringifyTerm(term.function);
        if (isLambda(term.function)) {
            func = '(' + func + ')';
        } 

        let arg = stringifyTerm(term.argument);
        if (isLambda(term.argument) || isApplication(term.argument)) {
            arg = '(' + arg + ')';
        } 

        return func + ' ' + arg;
    }

    if (isLambda(term)) {
        return '\\' + term.paramter + '. ' + stringifyTerm(term.body);
    } 

    throwNonLambda();
}

function createSvgElem(name) {
    return document.createElementNS('http://www.w3.org/2000/svg', name)
}

export function initSvgRoot(targetSvg) {
    document.body.addEventListener('touchstart', dragStart, false);
    document.body.addEventListener('touchend', dragEnd, false);
    document.body.addEventListener('touchmove', drag, false);

    document.body.addEventListener('mousedown', dragStart, false);
    document.body.addEventListener('mouseup', dragEnd, false);
    document.body.addEventListener('mousemove', drag, false);

    targetSvg.setAttribute(
        'width',
        targetSvg.width.baseVal.value
    );
    targetSvg.setAttribute(
        'height',
        targetSvg.height.baseVal.value
    );

    let current = { x: 0, y: 0 };
    let dragging = false;

    function move(offset) {
        let attribute = targetSvg.getAttribute('viewBox');
        let pieces;
        if (attribute == null || attribute == undefined) {
            pieces = [
                0,
                0,
                targetSvg.width.baseVal.value,
                targetSvg.height.baseVal.value,
            ];
        } else {
            pieces = attribute.split(' ').map(str => parseInt(str.trim()));
        }
        pieces[0] -= offset.x;
        pieces[1] -= offset.y;
        targetSvg.setAttribute('viewBox', pieces.join(' '));
    }

    function dragStart(evt) {
        if (evt.target == targetSvg) {
            
            if (evt.type == 'touchstart') {
                current.x = evt.touches[0].clientX;
                current.y = evt.touches[0].clientY;
            } else {
                current.x = evt.clientX;
                current.y = evt.clientY;
            }

            dragging = true;
        }
    }

    function dragEnd(evt) {
        dragging = false;
    }

    function drag(evt) {
        if (dragging) {
            evt.preventDefault();

            let offset = { x: 0, y: 0 };

            if (evt.type == 'touchstart') {
                offset.x = evt.touches[0].clientX - current.x;
                offset.y = evt.touches[0].clientY - current.y;
            } else {
                offset.x = evt.clientX - current.x;
                offset.y = evt.clientY - current.y;
            }

            move(offset);
            current.x += offset.x;
            current.y += offset.y;
        }
    }
}


export function defaultDrawConfig() {
    return {
        left: 40,
        top: 40,
        nodeRadius: 14,
        textTopBottom: 5,
        backgroundColor: '#ffffff',
        varColor: '#000000',
        lineColor: '#0000ff',
        lineWidth: 2,
        appColor: '#ff5000',
        lambdaColor: '#00a000',
        levelDistance: 50,
        leafDistance: 20,
    };
};

export function clearSvg(targetSvg, config) {
    targetSvg.replaceChildren();
}

export function drawTerm(term, targetSvg, config) {
    let actualConfig = Object.assign(defaultDrawConfig(), config);
    clearSvg(targetSvg, actualConfig);
    drawTermWith(term, targetSvg, actualConfig);
}

function cloneObj(obj) {
    return Object.assign({}, obj);
}

function mergeObj(objLeft, objRight) {
    return Object.assign(cloneObj(objLeft), objRight);
}

function nodeHeight(config) {
    return config.nodeRadius * 2 + 1;
}

function levelHeight(config) {
    return config.levelDistance + nodeHeight(config);
}

function drawBgCircle(config) {
    let circleNode = createSvgElem('circle');
    circleNode.setAttribute('r', config.nodeRadius);
    circleNode.setAttribute('stroke', config.backgroundColor);
    circleNode.setAttribute('fill', config.backgroundColor);
    circleNode.setAttribute('cx', 0);
    circleNode.setAttribute('cy', 0);
    return circleNode;
}

function drawNode(config, contentNode) {
    let gNode = createSvgElem('g');
    gNode.appendChild(drawBgCircle(config));
    gNode.appendChild(contentNode);

    let outerGNode = createSvgElem('g');
    outerGNode.appendChild(gNode);
    outerGNode.setAttribute(
        'transform',
        'translate(' + config.left + ',' + config.top + ')'
    );

    return outerGNode;
}

function drawLine(config, dx) {
    let lineNode = createSvgElem('line');
    lineNode.setAttribute('x1', 0);
    lineNode.setAttribute('y1', 0);
    lineNode.setAttribute('x2', dx);
    lineNode.setAttribute('y2', config.levelDistance);
    lineNode.setAttribute('stroke-width', config.lineWidth);
    lineNode.setAttribute('stroke', config.lineColor);
    let gNode = createSvgElem('g');
    gNode.appendChild(lineNode);
    gNode.setAttribute(
        'transform',
        'translate(' + config.left + ',' + config.top + ')'
    );
    return gNode;
}

function drawTermWith(term, parent, config) {
    if (isVariable(term)) {
        let gNode = createSvgElem('g');
        gNode.appendChild(drawBgCircle(config));
        let varNode = createSvgElem('text');
        varNode.setAttribute('fill', config.varColor);
        varNode.setAttribute('text-anchor', 'middle');
        varNode.setAttribute(
            'transform',
            'translate(0,' + config.textTopBottom + ')'
        );
        varNode.textContent = term.varname;
        varNode.setAttribute('class', 'lambda-drawing lambda-drawing-var');
        parent.appendChild(drawNode(config, varNode));
        return 1;
    }

    if (isApplication(term)) {
        let subConfig = cloneObj(config);
        subConfig.top += levelHeight(config);
        let leftLeafs = drawTermWith(term.function, parent, subConfig);

        subConfig.left += subConfig.leafDistance * (leftLeafs + 1);
        let rightLeafs = drawTermWith(term.argument, parent, subConfig);

        let nodeConfig = cloneObj(config);
        nodeConfig.left += Math.trunc(
            (leftLeafs + rightLeafs) * config.leafDistance / 2
        );

        let appNode = createSvgElem('text');
        appNode.setAttribute('fill', nodeConfig.appColor);
        appNode.setAttribute('text-anchor', 'middle');
        appNode.setAttribute(
            'transform',
            'translate(0,' + nodeConfig.textTopBottom + ')'
        );
        appNode.textContent = '@';
        appNode.setAttribute('class', 'lambda-drawing lambda-drawing-app');
        parent.appendChild(drawNode(nodeConfig, appNode));

        let lineConfig = nodeConfig;
        lineConfig.top += nodeHeight(config) / 2;

        parent.appendChild(drawLine(
            lineConfig,
            Math.trunc(-subConfig.leafDistance * (rightLeafs + leftLeafs) / 2)
        ));

        parent.appendChild(drawLine(
            lineConfig,
            Math.trunc(subConfig.leafDistance * (rightLeafs + leftLeafs) / 2)
        ));

        return leftLeafs + rightLeafs;
    }

    if (isLambda(term)) {
        let subTop = config.top + levelHeight(config);
        let subConfig = mergeObj(config, { top: subTop });
        let leafs = drawTermWith(term.body, parent, subConfig);

        let nodeConfig = cloneObj(config);
        nodeConfig.left += Math.trunc(leafs/2) * config.leafDistance;
        let lambdaNode = createSvgElem('text');
        lambdaNode.setAttribute('fill', nodeConfig.lambdaColor);
        lambdaNode.setAttribute('text-anchor', 'middle');
        lambdaNode.setAttribute(
            'transform',
            'translate(0,' + nodeConfig.textTopBottom + ')'
        );
        lambdaNode.textContent = 'λ' + term.parameter;
        lambdaNode.setAttribute('class', 'lambda-drawing lambda-drawing-lambda');
        parent.appendChild(drawNode(nodeConfig, lambdaNode));

        let lineConfig = nodeConfig;
        lineConfig.top += nodeHeight(config) / 2;

        parent.appendChild(drawLine(lineConfig, 0));

        return leafs;
    }

    throwNonLambda();
}
