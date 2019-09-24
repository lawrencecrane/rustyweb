const path = require('path');
const webpack = require("webpack");
const TerserJSPlugin = require('terser-webpack-plugin');

let babel_loader_rule = {
    test: /\.js$/,
    exclude: /node_modules/,
    use: {
        loader: "babel-loader",
        options: {
            presets: ["@babel/preset-env"]
        }
    }
};

module.exports = {
  mode: 'production',
  context: path.resolve(__dirname),
  optimization: {
    minimizer: [new TerserJSPlugin({})],
    minimize: true
  },
  entry: {
      app: path.resolve(__dirname, 'src/index.js')
  },
  output: {
    filename: 'bundle.js',
    path: path.resolve(__dirname, 'dist')
  },
  module: {
    rules: [babel_loader_rule]
  }
};
