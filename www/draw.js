import {
    isVariable,
    isApplication,
    isLambda,
    throwNonLambda
} from './lambda.js';

const classPrefix = 'drawing';

/**
 * Limpa um elemento SVG raiz.
 */
export function clearSvg(targetSvg) {
    targetSvg.replaceChildren();
}

/**
 * Desenha um termo lambda no dado elemento SVG raiz, usando as dadas mudanças
 * nas configurações. A raiz SVG precisa estar na página. Configurações
 * disponíveis:
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

    /**
     * Função chamada quando a janela é redimensionada, quando é necessário
     * redimensionar o SVG.
     */
    function resize() {
        let viewBox = targetSvg.getAttribute('viewBox');

        targetSvg.removeAttribute('width');
        targetSvg.removeAttribute('height');
        targetSvg.removeAttribute('viewBox');

        let pieces;
        if (viewBox == null) {
            pieces = [
                0,
                0,
                targetSvg.width.baseVal.value,
                targetSvg.height.baseVal.value,
            ];
        } else {
            pieces = viewBox.split(' ').map(str => parseInt(str.trim()));
            pieces[2] = targetSvg.width.baseVal.value;
            pieces[3] = targetSvg.height.baseVal.value;
        }

        targetSvg.setAttribute(
            'width',
            targetSvg.width.baseVal.value
        );
        targetSvg.setAttribute(
            'height',
            targetSvg.height.baseVal.value
        );
        targetSvg.setAttribute('viewBox', pieces.join(' '));
    }

    /**
     * "Move" o SVG pelo dado `offset` a partir da localização atual.
     */
    function move(offset) {
        let viewBox = targetSvg.getAttribute('viewBox');
        let pieces;
        if (viewBox == null) {
            pieces = [
                0,
                0,
                targetSvg.width.baseVal.value,
                targetSvg.height.baseVal.value,
            ];
        } else {
            pieces = viewBox.split(' ').map(str => parseInt(str.trim()));
        }
        pieces[0] -= offset.x;
        pieces[1] -= offset.y;
        targetSvg.setAttribute('viewBox', pieces.join(' '));
    }

    /**
     * Inicia um arrasto de imagem SVG.
     */
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

    /**
     * Finaliza um arrasto de imagem SVG.
     */
    function dragEnd(evt) {
        dragging = false;
    }

    /**
     * Faz um passo de arrasto de imagem SVG.
     */
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
 * Cria um elemento SVG qualquer no seu namespace correto.
 */
function createSvgElem(name) {
    return document.createElementNS('http://www.w3.org/2000/svg', name)
}

/**
 * Desenha um termo lambda dada uma config já ajustada.
 */
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

/**
 * Computa a largura de um nodo SVG.
 *
 * Se o nodo não está na página HTML, não é possível computar a largura dele.
 * Portanto, aqui é necessária uma gambiarra. Se `node` não está na imagem
 * SVG, inserimos o último ancestral de `node` na imagem, obtemos a largura, e
 * então removemos esse ancestral. Se ele já estiver na imagem, apenas obtemos
 * a largura. Note que para isso funcionar, a imagem PRECISA estar na página.
 */
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

/**
 * Desenha um termo do tipo variável na imagem.
 */
function drawVariable(term, targetSvg, config) {
    let textNode = createText(term.varname, 'variable', config.variable.node);
    let bgNode = createBg(
        textNode,
        targetSvg,
        'variable',
        config.variable.node
    );
    let wrapper = createNodeWrapper(textNode, bgNode, targetSvg, config);
    targetSvg.appendChild(wrapper);
    return config.symmetricPos(svgWidth(bgNode, targetSvg));
}

/**
 * Desenha um termo do tipo lambda na imagem.
 */
function drawLambda(term, targetSvg, config) {
    let textNode = createText(
        'λ' + term.parameter,
        'lambda',
        config.lambda.node
    );
    let bgNode = createBg(textNode, targetSvg, 'lambda', config.lambda.node);
    
    let childPos = drawLambdaBody(term.body, bgNode, targetSvg, config);
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

/**
 * Desenha o filho (corpo) de um lambda na imagem. `bgNode` é o 'fundo' SVG do
 * parâmetro do lambda. `term` já é o próprio corpo. Retorna a posição do
 * filho (ver classe `Position`).
 */
function drawLambdaBody(term, bgNode, targetSvg, config) {
    let childConfig = config.clone();
    childConfig.setMinCenter(
        config.symmetricCenter(svgWidth(bgNode, targetSvg))
    );
    childConfig.top += config.levelHeight('lambda');
    return drawTermWith(term, targetSvg, childConfig);
}

/**
 * Desenha o nodo do parâmetro do lambda (e.g. "λx"). Cria uma nova configuração
 * derivada para esse nodo e a retorna. `textNode` é o elemento SVG com o texto
 * do parâmetro, `bgNode` é o elemento com o 'fundo' do texto.
 */
function drawLambdaNode(textNode, bgNode, childPos, targetSvg, config) {
    let newConfig = config.clone();
    newConfig.minCenter = childPos.center;
    let wrapper = createNodeWrapper(textNode, bgNode, targetSvg, newConfig);
    targetSvg.appendChild(wrapper);
    return newConfig;
}

/**
 * Desenha a linha entre o parâmetro lambda e o corpo do lambda.
 */
function drawLambdaLine(position, targetSvg, config) {
    let line = createLine(
        position.center,
        config.top + config.lambda.node.radius,
        0,
        'lambda',
        config.lambda.line,
    );
    targetSvg.appendChild(line);
}

/**
 * Desenha um termo do tipo aplicação na imagem.
 */
function drawApplication(term, targetSvg, config) {
    let isRedex = isLambda(term.function);
    let configName, nodeClass;
    if (isRedex) {
        configName = 'redexApp';
        nodeClass = 'redex-app';
    } else {
        configName = 'nonRedexApp';
        nodeClass = 'non-redex-app';
    }

    let textNode = createText(
        '@',
        nodeClass,
        config[configName].node
    );
    let bgNode = createBg(
        textNode,
        targetSvg,
        nodeClass,
        config[configName].node
    );

    if (isRedex) {
        textNode.addEventListener('click', () => config.onredexclick(term));
        bgNode.addEventListener('click', () => config.onredexclick(term));
    }

    let leftChildPos = drawAppFunction(
        term.function,
        bgNode,
        targetSvg,
        configName,
        config,
    );

    let rightChildPos = drawAppArgument(
        term.argument,
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
        nodeClass,
        configName,
        newConfig,
    );

    return position;
}

/**
 * Desenha o filho-função de uma aplicação na imagem. `bgNode` é o 'fundo' SVG
 * do arroba de uma aplicação. `term` já é o próprio filho-função. `configName`
 * é ou 'redexApp' ou 'nonRedexApp'. Retorna a posição do filho
 * (ver classe `Position`).
 */
function drawAppFunction(term, bgNode, targetSvg, configName, config) {
    let leftChildConfig = config.clone();
    leftChildConfig.setMinCenter(
        config.symmetricCenter(svgWidth(bgNode, targetSvg))
        - config.minLeafDistance / 2
    );
    leftChildConfig.top += config.levelHeight(configName);
    return drawTermWith(term, targetSvg, leftChildConfig);
}

/**
 * Desenha o filho-argumento de uma aplicação na imagem. `bgNode` é o 'fundo'
 * SVG do arroba de uma aplicação. `term` já é o próprio filho-argumento.
 * `configName` é ou 'redexApp' ou 'nonRedexApp'. Retorna a posição do filho
 * (ver classe `Position`).
 */
function drawAppArgument(
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
    return drawTermWith(term, targetSvg, rightChildConfig);
}

/**
 * Desenha o nodo do arroba da aplicação (i.e. "@"). Cria uma nova configuração
 * derivada para esse nodo e a retorna. `textNode` é o elemento SVG com o texto
 * do parâmetro, `bgNode` é o elemento com o 'fundo' do texto.
 */
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

/**
 * Desenha as linhas entre o arroba de uma aplicação e seus filhos. `bgNode` é o
 * 'fundo' * SVG do arroba de uma aplicação. `configName` é ou 'redexApp' ou
 * 'nonRedexApp'.
 */
function drawAppLines(
    position,
    leftChildPos,
    rightChildPos,
    targetSvg,
    lineClass,
    configName,
    config,
) {
    drawAppLeftLine(
        position,
        leftChildPos,
        targetSvg,
        lineClass,
        configName,
        config
    );
    drawAppRightLine(
        position,
        rightChildPos,
        targetSvg,
        lineClass,
        configName,
        config
    );
}

/**
 * Desenha a linha entre o arroba da aplicação e o filho-função. `bgNode` é o
 * 'fundo' SVG do arroba de uma aplicação. `configName` é ou 'redexApp' ou
 * 'nonRedexApp'.
 */
function drawAppLeftLine(
    position,
    leftChildPos,
    targetSvg,
    lineClass,
    configName,
    config,
) {
    let leftLine = createLine(
        position.center,
        config.top + config.lambda.node.radius,
        leftChildPos.center - position.center,
        lineClass,
        config[configName].line,
    );
    targetSvg.appendChild(leftLine);
}

/**
 * Desenha a linha entre o arroba da aplicação e o filho-argumento. `bgNode` é o
 * 'fundo' SVG do arroba de uma aplicação. `configName` é ou 'redexApp' ou
 * 'nonRedexApp'.
 */
function drawAppRightLine(
    position,
    rightChildPos,
    targetSvg,
    lineClass,
    configName,
    config,
) {
    let rightLine = createLine(
        position.center,
        config.top + config.lambda.node.radius,
        rightChildPos.center - position.center,
        lineClass,
        config[configName].line,
    );
    targetSvg.appendChild(rightLine);
}

/**
 * Cria um nodo SVG de texto, com dado conteúdo, classe CSS, e configuração
 * do tipo específico de nodo.
 */
function createText(content, nodeClass, nodeConfig) {
    let textNode = createSvgElem('text');
    textNode.setAttribute('text-anchor', 'middle');
    textNode.setAttribute(
        'transform',
        'translate(0,' + nodeConfig.textPadding + ')'
    );
    textNode.textContent = content;
    textNode.setAttribute('class', classPrefix + "-" + nodeClass + "-text");
    return textNode;
}

/**
 * Cria um nodo SVG de fundo (elipse), para contornar o dado nodo de texto, com
 * configuração do tipo específico de nodo.
 */
function createBg(textNode, targetSvg, nodeClass, nodeConfig) {
    let ellipseNode = createSvgElem('ellipse');
    ellipseNode.setAttribute('rx', Math.max(
        nodeConfig.radius,
        nodeConfig.textPadding/2 + svgWidth(textNode, targetSvg),
    ));
    ellipseNode.setAttribute('ry', nodeConfig.radius);
    ellipseNode.setAttribute('cx', 0);
    ellipseNode.setAttribute('cy', 0);
    ellipseNode.setAttribute('class', classPrefix + "-" + nodeClass + "-bg");
    return ellipseNode;
}

/**
 * Cria uma linha saindo da dada posição à esquerda e ao tipo, indo `dx`
 * valores para à direita, e configuração do tipo específico da linha.
 */
function createLine(left, top, dx, lineClass, config) {
    let lineNode = createSvgElem('line');
    lineNode.setAttribute('x1', 0);
    lineNode.setAttribute('y1', 0);
    lineNode.setAttribute('x2', dx);
    lineNode.setAttribute('y2', config.height);
    lineNode.setAttribute('stroke-width', config.width);

    lineNode.setAttribute('class', classPrefix + '-' + lineClass + '-line');

    let gNode = createSvgElem('g');
    gNode.appendChild(lineNode);
    gNode.setAttribute(
        'transform',
        'translate(' + left + ',' + top + ')'
    );
    return gNode;
}

/**
 * Cria um "embrulho" sobre um nodo de texto e um nodo de fundo.
 */
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

/**
 * Posição de um nodo no desenho. Campos `left`, `center` e `right` todos
 * públicos para esse módulo.
 */
class Position {
    /**
     * Constrói essa posição com limite à esquerda e à direita, além do centro
     * entre os dois.
     */
    constructor(left, center, right) {
        this.left = left;
        this.center = center;
        this.right = right;
    }

    /**
     * Estende essa posição para abranger outras duas posições "prefixo" e
     * "sufixo".
     */
    extend(prefix, suffix) {
        this.left = Math.min(prefix.left, this.left);
        this.right = Math.max(this.right, suffix.right);
    }
}

/**
 * Classe que normaliza configurações. Todos campos públicos para esse módulo.
 */
class Config {
    /**
     * Dadas as mudanças nas configurações padrão, cria uma configuração.
     */
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

    /**
     * Reconfigura o centro mínimo dessa configuração. O antigo centro é
     * respeitado, então se o novo centro for menor, o antigo será usado.
     */
    setMinCenter(newMinCenter) {
        this.minCenter = Math.max(this.minCenter, newMinCenter);
    }

    /**
     * Computa a margem à esquerda real, compatível com o centro mínimo, dada a
     * largura do nodo em questão.
     */
    actualLeft(width) {
        let centerOffset = width / 2;
        return Math.max(this.left, this.minCenter - centerOffset);
    }

    /**
     * Computa o limite à direita, dada a largura do nodo em questão.
     */
    actualRight(width) {
        let left = this.actualLeft(width);
        return left + width;
    }

    /**
     * Computa o centro simétrico de um nodo com a dada largura, usando essa
     * configuração, respeitando o centro mínimo.
     */
    symmetricCenter(width) {
        let left = this.actualLeft(width);
        let centerOffset = width / 2;
        return left + centerOffset;
    }

    /**
     * Computa a posição com centro simétrico de um nodo com a dada largura,
     * usando essa configuração, respeitando o centro mínimo.
     */
    symmetricPos(width) {
        return new Position(
            this.actualLeft(width),
            this.symmetricCenter(width),
            this.actualRight(width),
        );
    }

    /**
     * Computa a altura de um nível, dada a chave identificando o tipo de termo.
     */
    levelHeight(key) {
        return (
            this[key].node.height
            + ('line' in this[key] ? this[key].line.height : 0)
        );
    }

    /**
     * Clona essa configuração num objeto completamente desvinculado.
     */
    clone() {
        return new this.constructor(this);
    }
}

/**
 * Configuração de nodos. Todos os campos são públicos para este módulo.
 */
class NodeConfig {
    /**
     * Dadas as mudanças nas configurações padrão, cria uma configuração de
     * nodo.
     */
    constructor(changes) {
        assignConfig(this, changes, {
            radius: 14,
            textPadding: 5,
            fillColor: '#ffffff',
            strokeColor: '#ffffff',
            textColor: '#000000',
        });
    }

    /**
     * Getter para a altura do nodo com essa configuração.
     */
    get height() {
        return this.radius * 2;
    }

    /**
     * Configuração padrão para nodo do tipo variável.
     */
    static defaultForVar(changes) {
        return new this(changes);
    }

    /**
     * Configuração padrão para nodo do tipo aplicação sem ser redex.
     */
    static defaultForNonRedexApp(changes) {
        return new this(assignConfig(Object.create(changes), changes, {
            textColor: '#ff6000',
        }));
    }

    /**
     * Configuração padrão para nodo do tipo redex.
     */
    static defaultForRedexApp(changes) {
        return new this(assignConfig(Object.create(changes), changes, {
            fillColor: '#e0e0ff',
            strokeColor: '#e0e0ff',
            textColor: '#ff0000',
        }));
    }

    /**
     * Configuração padrão para nodo do tipo lambda.
     */
    static defaultForLambda(changes) {
        return new this(assignConfig(Object.create(changes), changes, {
            textColor: '#008000',
        }));
    }
}

/**
 * Configuração para linhas. Campos públicos para esse módulo.
 */
class LineConfig {
    /**
     * Dadas as mudanças nas configurações padrão, cria uma configuração de
     * nodo.
     */
    constructor(changes) {
        assignConfig(this, changes, {
            width: 2,
            color: '#0000ff',
            height: 50,
        });
    }
}

/**
 * Atribui uma chave de configuração dentro de `dest`. Tenta-se usar o valor
 * em `source`, mas se não estiver disponível, usa-se `fallback`.
 */
function assignConfigKey(dest, key, source, fallback) {
    if (key in source) {
        dest[key] = source[key];
    } else {
        dest[key] = fallback;
    }
    return dest;
}

/**
 * Atribui todas as chaves de configuração vindas de `fallback` para dentro de
 * `dest`. Tenta-se usar o valor em `source`, mas se não estiver disponível,
 * usa-se `fallback`.
 */
function assignConfig(dest, source, fallback) {
    source = source || null;
    for (let key in fallback) {
        assignConfigKey(dest, key, source, fallback[key]);
    }
    return dest;
}
