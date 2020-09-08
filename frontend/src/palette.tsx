import * as React from 'react';
import * as ReactDOM from 'react-dom';

export class PalettePage extends React.Component {
	render() {
		return (
			<div>
				<PaletteHeader name="test" author="test"/>
			</div>
		)
	}
}

function PaletteHeader(props) {
	return (
		<div>
			<h1 className="palette_name">{props.name}</h1>
			<h4 className="palette_author">{props.author}</h4>
		</div>
	);
}

ReactDOM.render(
	<PalettePage/>,
	document.getElementById("palette_page")
)