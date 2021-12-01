import {
    isVariable,
    isApplication,
    isLambda,
    throwNonLambda
} from './lambda.js';

/**
 * Inicializa um elemento SVG raiz usado para desenhar termos lambda.
 *
 * È adicionado ao SVG a funcionalidade de arrastar, bem como de redimensionar
 * automaticamente.
 */
export function initSvgRoot(targetSvg) {
    document.body.addEventListener('touchstart', dragStart, false);
    document.body.addEventListener('touchend', dragEnd, false);
    document.body.addEventListener('touchmove', drag, false);
    document.body.addEventListener('mousedown', dragStart, false);
    document.body.addEventListener('mouseup', dragEnd, false);
    document.body.addEventListener('mousemove', drag, false);
    window.addEventListener('resize', resize);

    let current = { x: 0, y: 0 };
    let dragging = false;

    resize();

    function resize() {
        targetSvg.removeAttribute('width');
        targetSvg.removeAttribute('height');

        targetSvg.setAttribute(
            'width',
            targetSvg.width.baseVal.value
        );
        targetSvg.setAttribute(
            'height',
            targetSvg.height.baseVal.value
        );
    }

    function move(offset) {
        let attribute = targetSvg.getAttribute('viewBox');
        let pieces;
        if (attribute == null) {
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

            if (evt.type == 'touchmove') {
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

/**
 * Limpa um elemento SVG raiz.
 */
export function clearSvg(targetSvg) {
    targetSvg.replaceChildren();
}

/**
 * Desenha um termo lambda no dado elemento SVG raiz, usando as dadas mudanças
 * nas configurações. Configurações disponíveis:
 * 
 * chave: valor padrão // explicação
 *
 * top: 40                                  // margem superior
 * left: 40                                 // margem à esquerda
 * minCenter: 0                             // posição mínima do centro
 * minLeafDistance: 20                      // distância mínima entre nodos folhas
 * onredexclick: term => console.log(term)  // evento disparado ao clicar em redexes
 * variable.node.radius: 14                 // raio mínimo do contorno dos nodos de variáveis
 * variable.node.textPadding: 5             // espaçamento ao redor do texto de nodos de variáveis
 * variable.node.fillColor: '#ffffff'       // cor de preenchimento do fundo de nodos de variáveis
 * variable.node.strokeColor: '#ffffff'     // cor de contorno do fundo de nodos de variáveis
 * variable.node.textColor: '#000000'       // cor do texto dos nodos de variáveis
 * lambda.node.radius: 14                   // raio mínimo do contorno dos nodos de lambdas
 * lambda.node.textPadding: 5               // espaçamento ao redor do texto de nodos de lambdas
 * lambda.node.fillColor: '#ffffff'         // cor de preenchimento do fundo de nodos de lambdas
 * lambda.node.strokeColor: '#ffffff'       // cor de contorno do fundo de nodos de lambdas
 * lambda.node.textColor: '#008000'         // cor do texto dos nodos de lambdas
 * lambda.line.width: 2                     // grossura da linha entre lambdas e seus filhos
 * lambda.line.color: '#0000ff'             // cor da linha entre lambdas e seus filhos
 * lambda.line.height: 50                   // altura da linha entre lambdas e seus filhos
 * nonRedexApp.node.radius: 14              // raio mínimo do contorno dos nodos de aplicações
 * nonRedexApp.node.textPadding: 5          // espaçamento ao redor do texto de nodos de aplicações
 * nonRedexApp.node.fillColor: '#ffffff'    // cor de preenchimento do fundo de nodos de aplicações 
 * nonRedexApp.node.strokeColor: '#ffffff'  // cor de contorno do fundo de nodos de aplicações
 * nonRedexApp.node.textColor: '#ff6000'    // cor do texto dos nodos de aplicações
 * nonRedexApp.line.width: 2                // grossura da linha entre aplicações e seus filhos
 * nonRedexApp.line.color: '#0000ff'        // cor da linha entre aplicações e seus filhos
 * nonRedexApp.line.height: 50              // altura da linha entre aplicações e seus filhos
 * redexApp.node.radius: 14                 // raio mínimo do contorno dos nodos de redexes
 * redexApp.node.textPadding: 5             // espaçamento ao redor do texto de nodos de redexes
 * redexApp.node.fillColor: '#e0e0ff'       // cor de preenchimento do fundo de nodos de redexes 
 * redexApp.node.strokeColor: '#e0e0ff'     // cor de contorno do fundo de nodos de redexes
 * redexApp.node.textColor: '#ff0000'       // cor do texto dos nodos de redexes
 * redexApp.line.width: 2                   // grossura da linha entre redexes e seus filhos
 * redexApp.line.color: '#0000ff'           // cor da linha entre redexes e seus filhos
 * redexApp.line.height: 50                 // altura da linha entre redexes e seus filhos
 */
export function drawTerm(term, targetSvg, configChanges) {
    clearSvg(targetSvg);
    drawTermWith(term, targetSvg, new Config(configChanges));
}

function createSvgElem(name) {
    return document.createElementNS('http://www.w3.org/2000/svg', name)
}

function drawTermWith(term, targetSvg, config) {
    if (isVariable(term)) {
        return drawVariable(term, targetSvg, config);
    }

    if (isApplication(term)) {
        return drawApplication(term, targetSvg, config);
    }

    if (isLambda(term)) {
        return drawLambda(term, targetSvg, config);
    }

    throwNonLambda();
}

function svgWidth(node, targetSvg) {
    let inserted = false;
    let element = node;
    while (!inserted && element.parentNode != null) {
        if (element.parentNode == targetSvg) {
            inserted = true;
        } else {
            element = element.parentNode;
        }
    }
    if (!inserted) {
        targetSvg.appendChild(element);
    }
    let width = node.getBBox().width;
    if (!inserted) {
        targetSvg.removeChild(element);
    }
    if (width == 0) {
        throw new Error("Element with zero width");
    }
    return width;
}

function drawVariable(term, targetSvg, config) {
    let textNode = createText(
        term.varname,
        'lambda-drawing-variable',
        config.variable.node
    );
    let bgNode = createBg(textNode, targetSvg, config.variable.node);
    let wrapper = createNodeWrapper(textNode, bgNode, targetSvg, config);
    targetSvg.appendChild(wrapper);
    return config.symmetricPos(svgWidth(bgNode, targetSvg));
}

function drawLambda(term, targetSvg, config) {
    let textNode = createText(
        'λ' + term.parameter,
        'lambda-drawing-lambda',
        config.lambda.node
    );
    let bgNode = createBg(textNode, targetSvg, config.lambda.node);
    
    let childPos = drawLambdaChild(term, bgNode, targetSvg, config);
    let newConfig = drawLambdaNode(
        textNode,
        bgNode,
        childPos,
        targetSvg,
        config
    );

    let position = newConfig.symmetricPos(svgWidth(bgNode, targetSvg));
    position.extend(childPos, childPos);

    drawLambdaLine(position, targetSvg, newConfig);

    return position;
}

function drawLambdaChild(term, bgNode, targetSvg, config) {
    let childConfig = config.clone();
    childConfig.setMinCenter(
        config.symmetricCenter(svgWidth(bgNode, targetSvg))
    );
    childConfig.top += config.levelHeight('lambda');
    return drawTermWith(term.body, targetSvg, childConfig);
}

function drawLambdaNode(textNode, bgNode, childPos, targetSvg, config) {
    let newConfig = config.clone();
    newConfig.minCenter = childPos.center;
    let wrapper = createNodeWrapper(textNode, bgNode, targetSvg, newConfig);
    targetSvg.appendChild(wrapper);
    return newConfig;
}

function drawLambdaLine(position, targetSvg, config) {
    let line = createLine(
        position.center,
        config.top + config.lambda.node.radius,
        0,
        config.lambda.line,
    );
    targetSvg.appendChild(line);
}

function drawApplication(term, targetSvg, config) {
    let isRedex = isLambda(term.function);
    let configName, nodeClass;
    if (isRedex) {
        configName = 'redexApp';
        nodeClass = 'lambda-drawing-redex-app';
    } else {
        configName = 'nonRedexApp';
        nodeClass = 'lambda-drawing-non-redex-app';
    }

    let textNode = createText(
        '@',
        'lambda-drawing-lambda ' + nodeClass,
        config[configName].node
    );
    let bgNode = createBg(textNode, targetSvg, config[configName].node);
    if (isRedex) {
        textNode.addEventListener('click', () => config.onredexclick(term));
        bgNode.addEventListener('click', () => config.onredexclick(term));
    }

    let leftChildPos = drawAppLeftChild(
        term,
        bgNode,
        targetSvg,
        configName,
        config,
    );

    let rightChildPos = drawAppRightChild(
        term,
        bgNode,
        leftChildPos,
        targetSvg,
        configName,
        config,
    );

    let newConfig = drawAppNode( 
        textNode,
        bgNode,
        leftChildPos,
        rightChildPos,
        targetSvg,
        config,
    );

    let position = newConfig.symmetricPos(svgWidth(bgNode, targetSvg));
    position.extend(leftChildPos, rightChildPos);
    
    drawAppLines(
        position,
        leftChildPos,
        rightChildPos,
        targetSvg,
        configName,
        newConfig,
    );

    return position;
}

function drawAppLeftChild(term, bgNode, targetSvg, configName, config) {
    let leftChildConfig = config.clone();
    leftChildConfig.setMinCenter(
        config.symmetricCenter(svgWidth(bgNode, targetSvg))
        - config.minLeafDistance / 2
    );
    leftChildConfig.top += config.levelHeight(configName);
    return drawTermWith(term.function, targetSvg, leftChildConfig);
}

function drawAppRightChild(
    term,
    bgNode,
    leftChildPos,
    targetSvg,
    configName,
    config,
) {
    let rightChildConfig = config.clone();
    rightChildConfig.left = leftChildPos.right + config.minLeafDistance;
    rightChildConfig.setMinCenter(rightChildConfig.left);
    rightChildConfig.top += config.levelHeight(configName);
    return drawTermWith(term.argument, targetSvg, rightChildConfig);
}

function drawAppNode(
    textNode,
    bgNode,
    leftChildPos,
    rightChildPos,
    targetSvg,
    config,
) {
    let newConfig = config.clone();
    newConfig.setMinCenter(
        (leftChildPos.center + rightChildPos.center) / 2
    );
    let wrapper = createNodeWrapper(textNode, bgNode, targetSvg, newConfig);
    targetSvg.appendChild(wrapper);
    return newConfig;
}

function drawAppLines(
    position,
    leftChildPos,
    rightChildPos,
    targetSvg,
    configName,
    config,
) {
    drawAppLeftLine(position, leftChildPos, targetSvg, configName, config);
    drawAppRightLine(position, rightChildPos, targetSvg, configName, config);
}

function drawAppLeftLine(
    position,
    leftChildPos,
    targetSvg,
    configName,
    config,
) {
    let leftLine = createLine(
        position.center,
        config.top + config.lambda.node.radius,
        leftChildPos.center - position.center,
        config[configName].line,
    );
    targetSvg.appendChild(leftLine);
}

function drawAppRightLine(
    position,
    rightChildPos,
    targetSvg,
    configName,
    config,
) {
    let rightLine = createLine(
        position.center,
        config.top + config.lambda.node.radius,
        rightChildPos.center - position.center,
        config[configName].line,
    );
    targetSvg.appendChild(rightLine);
}

function createText(content, nodeClass, nodeConfig) {
    let textNode = createSvgElem('text');
    textNode.setAttribute('fill', nodeConfig.textColor);
    textNode.setAttribute('text-anchor', 'middle');
    textNode.setAttribute(
        'transform',
        'translate(0,' + nodeConfig.textPadding + ')'
    );
    textNode.textContent = content;
    textNode.setAttribute('class', 'lambda-drawing ' + nodeClass);
    return textNode;
}

function createBg(textNode, targetSvg, nodeConfig) {
    let ellipseNode = createSvgElem('ellipse');
    ellipseNode.setAttribute('rx', Math.max(
        nodeConfig.radius,
        nodeConfig.textPadding/2 + svgWidth(textNode, targetSvg),
    ));
    ellipseNode.setAttribute('ry', nodeConfig.radius);
    ellipseNode.setAttribute('fill', nodeConfig.fillColor);
    ellipseNode.setAttribute('stroke', nodeConfig.strokeColor);
    ellipseNode.setAttribute('cx', 0);
    ellipseNode.setAttribute('cy', 0);
    return ellipseNode;
}

function createLine(left, top, dx, config) {
    let lineNode = createSvgElem('line');
    lineNode.setAttribute('x1', 0);
    lineNode.setAttribute('y1', 0);
    lineNode.setAttribute('x2', dx);
    lineNode.setAttribute('y2', config.height);
    lineNode.setAttribute('stroke-width', config.width);
    lineNode.setAttribute('stroke', config.color);
    let gNode = createSvgElem('g');
    gNode.appendChild(lineNode);
    gNode.setAttribute(
        'transform',
        'translate(' + left + ',' + top + ')'
    );
    return gNode;
}

function createNodeWrapper(textNode, bgNode, targetSvg, config) {
    let gNode = createSvgElem('g');
    gNode.appendChild(bgNode);
    gNode.appendChild(textNode);

    let outerGNode = createSvgElem('g');
    outerGNode.appendChild(gNode);
    let center = config.symmetricCenter(svgWidth(bgNode, targetSvg));
    outerGNode.setAttribute(
        'transform',
        'translate(' + center + ',' + config.top + ')'
    );

    return outerGNode;
}

class Position {
    constructor(left, center, right) {
        this.left = left;
        this.center = center;
        this.right = right;
    }

    extend(prefix, suffix) {
        this.left = Math.min(prefix.left, this.left);
        this.right = Math.max(this.right, suffix.right);
    }
}

class Config {
    constructor(changes) {
        changes = changes || {};
        
        assignConfig(this, changes, {
            top: 40,
            left: 40,
            minCenter: 0,
            minLeafDistance: 20,
            onredexclick: term => console.log(term),
        });
        this.variable = {
            node: NodeConfig.defaultForVar(
                'variable' in changes ? changes.variable.node : {}
            ),
        };
        this.nonRedexApp = {
            node: NodeConfig.defaultForNonRedexApp(
                'nonRedexApp' in changes ? changes.nonRedexApp.node : {}
            ),
            line: new LineConfig(
                'nonRedexApp' in changes ? changes.nonRedexApp.line : {}
            ),
        };
        this.redexApp = {
            node: NodeConfig.defaultForRedexApp(
                'redexApp' in changes ? changes.redexApp.node : {}
            ),
            line: new LineConfig(
                'redexApp' in changes ? changes.redexApp.line : {}
            ),
        };
        this.lambda = {
            node: NodeConfig.defaultForLambda(
                'lambda' in changes ? changes.lambda.node : {}
            ),
            line: new LineConfig(
                'lambda' in changes ? changes.lambda.line : {}
            ),
        };
    }

    setMinCenter(newMinCenter) {
        this.minCenter = Math.max(this.minCenter, newMinCenter);
    }

    actualLeft(width) {
        let centerOffset = width / 2;
        return Math.max(this.left, this.minCenter - centerOffset);
    }

    actualRight(width) {
        let left = this.actualLeft(width);
        return left + width;
    }

    symmetricCenter(width) {
        let left = this.actualLeft(width);
        let centerOffset = width / 2;
        return left + centerOffset;
    }

    symmetricPos(width) {
        return new Position(
            this.actualLeft(width),
            this.symmetricCenter(width),
            this.actualRight(width),
        );
    }

    levelHeight(key) {
        return (
            this[key].node.height
            + ('line' in this[key] ? this[key].line.height : 0)
        );
    }

    clone() {
        return new this.constructor(this);
    }
}

class NodeConfig {
    constructor(changes) {
        assignConfig(this, changes, {
            radius: 14,
            textPadding: 5,
            fillColor: '#ffffff',
            strokeColor: '#ffffff',
            textColor: '#000000',
        });
    }

    get height() {
        return this.radius * 2;
    }

    static defaultForVar(changes) {
        return new this(changes);
    }

    static defaultForNonRedexApp(changes) {
        return new this(assignConfig(Object.create(changes), changes, {
            textColor: '#ff6000',
        }));
    }

    static defaultForRedexApp(changes) {
        return new this(assignConfig(Object.create(changes), changes, {
            fillColor: '#e0e0ff',
            strokeColor: '#e0e0ff',
            textColor: '#ff0000',
        }));
    }

    static defaultForLambda(changes) {
        return new this(assignConfig(Object.create(changes), changes, {
            textColor: '#008000',
        }));
    }
}

class LineConfig {
    constructor(changes) {
        assignConfig(this, changes, {
            width: 2,
            color: '#0000ff',
            height: 50,
        });
    }
}

function assignConfigKey(dest, key, source, fallback) {
    if (key in source) {
        dest[key] = source[key];
    } else {
        dest[key] = fallback;
    }
    return dest;
}

function assignConfig(dest, source, fallback) {
    source = source || null;
    for (let key in fallback) {
        assignConfigKey(dest, key, source, fallback[key]);
    }
    return dest;
}
