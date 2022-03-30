async function convertToHtml(
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	virtualNode: Record<string, any> | string | number
) {
	if (typeof virtualNode === 'string' || typeof virtualNode === 'number') {
		return document.createTextNode(`${virtualNode}`);
	}

	const $DOMElement = document.createElement(
		virtualNode.tagName
	) as HTMLElement;

	const propsWithoutChildren = Object.entries(virtualNode.props).filter(
		([key]) => key !== 'children'
	);

	for await (const [key, value] of propsWithoutChildren) {
		if (key === 'style') {
			for (const [cssPropertyKey, cssPropertyValue] of Object.entries(value)) {
				$DOMElement.style[cssPropertyKey] = cssPropertyValue;
			}
		} else {
			if (key.startsWith('on') && typeof value === 'function') {
				const eventName = key.substring(2).toLowerCase();
				const eventHandler = value as EventListenerOrEventListenerObject;

				$DOMElement.addEventListener(eventName, eventHandler);
			} else {
				$DOMElement.setAttribute(key, `${value}`);
			}
		}
	}

	for await (const virtualChild of virtualNode.props.children) {
		$DOMElement.append(await convertToHtml(virtualChild));
	}

	return $DOMElement;
}

async function render(
	initialVirtualTree: Record<string, unknown>,
	$DOMRoot: HTMLDivElement
) {
	const $AppHTML = await convertToHtml(initialVirtualTree);
	$DOMRoot.append($AppHTML);
}

const ReactDOM = {
	render
};

export default ReactDOM;
