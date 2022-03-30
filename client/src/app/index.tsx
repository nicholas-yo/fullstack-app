import React from '../lib/React/index.js';

interface InputProps {
	type: string;
	name: string;
	placeholder: string;
	id: string;
}

const Head = () => (
	<head>
		<title>My App</title>
	</head>
);

const Input = ({ name, placeholder, type, id }: InputProps) => (
	<input type={type} name={name} id={id} placeholder={placeholder} required />
);

const Button = () => <button type='submit'>Enviar</button>;

const Form = () => {
	const url = '/api/register';

	return (
		<form action={url} method='post'>
			<Input name='name' id='name' placeholder='name' type='text' />
			<Input
				name='email'
				id='email'
				placeholder='email@example.com'
				type='email'
			/>
			<Input
				name='password'
				id='password'
				placeholder='password'
				type='password'
			/>
			<Input
				id='confirm_password'
				name='confirm_password'
				placeholder='confirm password'
				type='password'
			/>
			<Button />
		</form>
	);
};

const App = () => (
	<div>
		<Head />

		<Form />
	</div>
);

export default App;
