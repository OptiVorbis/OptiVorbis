/* eslint-disable @typescript-eslint/no-require-imports */

const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const FaviconsWebpackPlugin = require('favicons-webpack-plugin');

const dist = path.resolve(__dirname, '..', 'target', 'web');

module.exports = {
  mode: 'production',
  entry: './src/ts/main.ts',
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: 'ts-loader',
        exclude: /node_modules/,
        include: path.resolve(__dirname, 'src', 'ts'),
      },
      {
        test: /\.css$/,
        use: [MiniCssExtractPlugin.loader, 'css-loader', 'postcss-loader'],
      },
      {
        test: /\.png$/,
        type: 'asset/resource',
      },
      {
        test: /\.svg$/,
        use: 'svg-inline-loader',
      },
    ],
  },
  output: {
    path: dist,
    clean: true,
  },
  resolve: {
    extensions: ['.ts', '...'],
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: './src/index.html',
    }),
    new FaviconsWebpackPlugin({
      logo: './src/static/optivorbis_logo.png',
      mode: 'webapp',
      devMode: 'webapp',
      favicons: {
        appName: 'OptiVorbis',
        appDescription: 'A web demo application for OptiVorbis, a library for lossless optimization and repair of Ogg Vorbis files.',
        theme_color: '#45718c',
        icons: {
          yandex: false,
        },
      },
    }),
    new MiniCssExtractPlugin(),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, '..', 'packages', 'optivorbis'),
      outDir: '../../target/webpkg',
      extraArgs: '-- --features=wasm-web-bindings -Z build-std=panic_abort,core,std,alloc,proc_macro -Z build-std-features=panic_immediate_abort,optimize_for_size',
      forceMode: 'production',
    }),
  ],
  experiments: {
    asyncWebAssembly: true,
  },
};
