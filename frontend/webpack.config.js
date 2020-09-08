module.exports = {
	entry: './src/',
	output: {
		path: __dirname + '/public'
	},
	resolve: {
		extensions: ['.ts', '.tsx', '.js']
	},
	module: {
		rules: [{
			test: /\.tsx?$/,
			loader: 'ts-loader'
		}]
	}
}