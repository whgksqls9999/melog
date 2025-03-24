import { BrowserRouter } from 'react-router-dom';
import { Router } from './router';
import { Setter } from './setter';

function Application() {
	return (
		<BrowserRouter>
			<Setter>
				<Router />
			</Setter>
		</BrowserRouter>
	);
}

export default Application;
