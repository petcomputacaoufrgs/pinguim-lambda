import * as styles from './styles.css';
import * as commonStyles from './common_styles.css';

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
    return Document.createElementNs('http://www.w3.org/2000/svg', name)
}

export function initSvgRoot(targetSvg) {
}

export function clearSvg(targetSvg) {
    targetSvg.replaceChildren();
}

export function defaultDrawConfig() {
    return {
        left: 40,
        top: 40,
        nodeRadius: 14,
        textPaddingTop: 5,
        backgroundColor: '#ffffff',
        textColor: '#000000',
        lineColor: '#0000ff',
        lineWidth: 2,
        applicationColor: '#ff8000',
        levelDistance: 50,
        leafDistance: 20,
    };
};

export function drawTerm(term, targetSvg, config) {
    clearSvg(targetSvg);
    drawTermWith(term, targetSvg, Object.assign(defaultDrawConfig(), config));
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

function drawLine(config, dx, dy) {
    let lineNode = createSvgElem('line');
    lineNode.setAttribute('x1', 0);
    lineNode.setAttribute('y1', 0);
    lineNode.setAttribute('x2', dx);
    lineNode.setAttribute('y2', dy);
    lineNode.setAttribute('stroke-width', subConfig.lineWidth);
    lineNode.setAttribute('stroke', subConfig.lineColor);
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
        varNode.setAttribute('fill', config.textColor);
        varNode.setAttribute('text-anchor', 'middle');
        varNode.setAttribute(
            'transform',
            'translate(0,' + config.textPaddingTop + ')'
        );
        varNode.textContent = term.varname;
        varNode.className = 'lambda-drawing lambda-drawing-text';
        parent.appendChild(drawNode(config, varNode));
        return 1;
    }

    if (isApplication(term)) {
        let subTop = config.top + levelHeight(config);
        let subConfig = mergeObj(config, { top: subTop });
        let leftLeafs = drawTermWith(term.function, parent, subConfig);

        subconfig.left += subconfig.leafDistance * (leftLeafs + 1);
        let rightLeafs = drawTermWith(term.argument, parent, subConfig);

        let nodeConfig = cloneObj(config);
        nodeConfig.left += Math.trunc(
            (leftLeafs + rightLeafs) * config.leafDistance / 2
        );

        let appNode = createSvgElem('text');
        appNode.setAttribute('fill', nodeConfig.applicationColor);
        appNode.setAttribute('text-anchor', 'middle');
        appNode.setAttribute(
            'transform',
            'translate(0,' + nodeConfig.appPaddingTop + ')'
        );
        appNode.appContent = '@';
        appNode.className = 'lambda-drawing lambda-drawing-app';
        parent.appendChild(drawNode(nodeConfig, appNode));

        let lineConfig = nodeConfig;
        lineConfig.top += nodeHeight(config);

        parent.appendChild(drawLine(
            config,
            Math.trunc(-subConfig.leafDistance * (rightLeafs + leftLeafs) / 4)
            lineConfig
        ));

        parent.appendChild(drawLine(
            config,
            Math.trunc(subConfig.leafDistance * (rightLeafs + leftLeafs) / 4)
            lineConfig
        ));

        return leftLeafs + rightLeafs;
    }

    if (isLambda(term)) {
        let subTop = config.top + levelHeight(config);
        let subConfig = mergeObj(config, { top: subTop });
        let leafs = drawTermWith(term.body, parent, subConfig);

        let nodeConfig = cloneObj(config);
        nodeConfig.left += Math.trunc(
            (leftLeafs + 1) * config.leafDistance / 2
        );
        let lambdaNode = createSvgElem('text');
        lambdaNode.setAttribute('fill', nodeConfig.lambdalicationColor);
        lambdaNode.setAttribute('text-anchor', 'middle');
        lambdaNode.setAttribute(
            'transform',
            'translate(0,' + nodeConfig.lambdaPaddingTop + ')'
        );
        lambdaNode.lambdaContent = 'λ' + term.parameter;
        lambdaNode.className = 'lambda-drawing lambda-drawing-lambda';
        parent.appendChild(drawNode(nodeConfig, lambdaNode));

        let lineConfig = nodeConfig;
        lineConfig.top += nodeHeight(config);

        parent.appendChild(drawLine(config, 0, lineConfig));

        return leafs;
    }
}
