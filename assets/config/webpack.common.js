'use strict'; // eslint-disable-line

const path = require('path');
const webpack = require('webpack');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const StyleLintPlugin = require('stylelint-webpack-plugin');
const ESLintPlugin = require('eslint-webpack-plugin');
const { WebpackManifestPlugin } = require('webpack-manifest-plugin');
const RemoveEmptyScriptsPlugin = require('webpack-remove-empty-scripts');
const { VueLoaderPlugin } = require('vue-loader');

const rootPath = process.cwd();

const assetPath = path.join(rootPath, 'assets');
const distPath = path.join(rootPath, 'static');
const publicPath = '/static/';

module.exports = {
  context: assetPath,

  entry: './index.js',

  output: {
    path: distPath,
    filename: 'scripts/[name].js',
    publicPath: publicPath,
  },

  optimization: {
    usedExports: true,
    splitChunks: {
      cacheGroups: {
        commons: {
          test: /[\\/]node_modules[\\/]/,
          name: 'vendors',
          chunks: 'all',
        },
      },
    },
  },

  module: {
    rules: [
      {
        test: /\.js$/,
        exclude: /(node_modules)/,
        use: [
          { loader: 'swc-loader' },
        ],
      },
      {
        test: /\.vue$/,
        loader: 'vue-loader',
      },
      {
        test: /\.scss$/,
        include: assetPath,
        use: [
          MiniCssExtractPlugin.loader,
          { loader: 'css-loader', options: { sourceMap: true } },
          { loader: 'postcss-loader', options: { sourceMap: true } },
          { loader: 'resolve-url-loader', options: { sourceMap: true } },
          { loader: 'sass-loader', options: { sourceMap: true } },
        ],
      },
    ],
  },

  resolve: {
    alias: {
      'vue': 'vue/dist/vue.esm-bundler',
      'vuex': 'vuex/dist/vuex.esm-bundler',
    },
  },

  plugins: [
    new CleanWebpackPlugin(),
    new ESLintPlugin(),
    new RemoveEmptyScriptsPlugin(),
    new WebpackManifestPlugin({
      fileName: 'assets-manifest.json',
      publicPath: '',
    }),
    new StyleLintPlugin({
      failOnError: false,
      customSyntax: 'postcss-scss',
    }),
    new VueLoaderPlugin(),
    new webpack.DefinePlugin({
      __VUE_OPTIONS_API__: true,
      __VUE_PROD_DEVTOOLS__: false,
    }),
  ],

};
