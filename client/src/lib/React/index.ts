function createElement(
	elementType: (arg: Record<string, unknown>) => void | string,
	props?: Record<string, unknown>,
	...children: Array<HTMLElement | string>
) {
	const virtualElementProps = {
		...props,
		children
	};

	if (typeof elementType === 'function')
		return elementType(virtualElementProps);

	return {
		tagName: elementType,
		props: virtualElementProps
	};
}

const React = {
	createElement
};

export default React;
