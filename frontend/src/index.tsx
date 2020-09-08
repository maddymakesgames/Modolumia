import * as React from 'react';
import * as ReactDOM from 'react-dom';

ReactDOM.render(
  <div>Hello world</div>,
  document.getElementById("root"),
);

ReactDOM.render(
	<Header />,
	document.getElementById("header")
);


function Header(props) {
	return (
		<div>
			<button className="header_button">Palettes</button>
			<button className="header_button">Music Packs</button>;
			<button className="header_button">Leaderboard</button>;
			<button className="header_button">Login</button>;
		</div>
	);
}