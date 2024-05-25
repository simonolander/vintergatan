const path = require('path');
const CopyPlugin = require('copy-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

const dist = path.resolve(__dirname, 'dist');

module.exports = {
  mode: 'production',
  entry: {
    index: './js/index.js',
  },
  experiments: {
    asyncWebAssembly: true,
  },
  output: {
    path: dist,
    filename: '[name].js',
  },
  devServer: {
    static: {
      directory: path.resolve(__dirname, 'static'),
    },
  },
  plugins: [
    new CopyPlugin({
      patterns: [
        path.resolve(__dirname, 'static'),
      ],
    }),

    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ],
};
