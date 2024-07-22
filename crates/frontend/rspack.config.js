'use strict';

const path = require('path');
const rspack = require('@rspack/core');
const { RspackManifestPlugin } = require('rspack-manifest-plugin');
const { VueLoaderPlugin } = require('vue-loader');
const ESLintPlugin = require('eslint-rspack-plugin');
const StyleLintPlugin = require('stylelint-webpack-plugin');

const rootPath = process.cwd();

const assetPath = path.join(rootPath, 'assets');
const distPath = path.join(rootPath, 'static');
const publicPath = '/static/';

const isProduction = process.env.NODE_ENV === 'production';

/** @type {import('@rspack/cli').Configuration} */
const config= {
  mode: isProduction ? 'production' : 'development',

  context: assetPath,

  entry: './index.js',

  output: {
    clean: true,
    path: distPath,
    filename: isProduction ? 'scripts/[name].[contenthash:8].js' : 'scripts/[name].js',
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

  experiments: {
    css: false,
  },

  module: {
    rules: [
      {
        test: /\.([jt])s$/,
        exclude: [/[\\/]node_modules[\\/]/],
        use: [
          { loader: 'builtin:swc-loader' },
        ],
      },
      {
        test: /\.vue$/,
        loader: 'vue-loader',
        options: {
          experimentalInlineMatchResource: true,
        },
      },
      {
        test: /\.scss$/,
        include: assetPath,
        use: [
          rspack.CssExtractRspackPlugin.loader,
          { loader: 'css-loader', options: { sourceMap: true } },
          { loader: 'postcss-loader', options: { sourceMap: true } },
          { loader: 'resolve-url-loader', options: { sourceMap: true } },
          { loader: 'sass-loader', options: {
            sourceMap: true,
            api: 'modern-compiler',
            implementation: require.resolve('sass-embedded'),
            sassOptions: {
              quietDeps: true, // Disable warning when building dependencies
            },
          } },
        ],
        type: 'javascript/auto',
      },
      {
        test: /\.(png|jpe?g|gif|svg|ico)$/,
        type: 'asset/resource',
        generator: {
          filename: isProduction ? 'images/[name].[contenthash:8][ext][query]' : 'images/[name][ext][query]',
        },
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
    new rspack.DefinePlugin({
      __VUE_OPTIONS_API__: true,
      __VUE_PROD_DEVTOOLS__: false,
      __VUE_PROD_HYDRATION_MISMATCH_DETAILS__: true,
    }),

    new RspackManifestPlugin({
      fileName: 'assets-manifest.json',
      publicPath: '',
    }),

    new rspack.CssExtractRspackPlugin({
      filename: isProduction ? 'styles/[name].[contenthash:8].css' : 'styles/[name].css',
      chunkFilename: 'styles/[id].css',
    }),

    new rspack.CopyRspackPlugin({
      patterns: [
        {
          from: 'images',
          to: isProduction ? 'images/[path][name].[contenthash:8][ext]' : 'images/[path][name].[ext]',
        },
      ],
    }),

    new VueLoaderPlugin(),

    new ESLintPlugin(),

    new StyleLintPlugin({
      failOnError: false,
      customSyntax: 'postcss-scss',
    }),
  ],
};

module.exports = config;
