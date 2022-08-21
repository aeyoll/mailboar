const glob = require('glob-all');
const { merge } = require('webpack-merge');
const ImageminPlugin = require('imagemin-webpack-plugin').default;
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const CopyPlugin = require('copy-webpack-plugin');
const { PurgeCSSPlugin } = require('purgecss-webpack-plugin');

const common = require('./webpack.common.js');

const purgeFromVue = (content) => {
  const contentWithoutStyleBlocks = content.replace(/<style[^]+?<\/style>/gi, '');
  return contentWithoutStyleBlocks.match(/[A-Za-z0-9-_/:]*[A-Za-z0-9-_/]+/g) || [];
};

module.exports = merge(common, {
  mode: 'production',
  devtool: false,
  output: {
    filename: 'scripts/[name].[contenthash:8].js',
  },
  module: {
    rules: [
      {
        test: /\.(png|jpe?g|gif|svg|ico)$/,
        type: 'asset/resource',
        generator: {
          filename: 'images/[name].[contenthash:8][ext][query]',
        },
      },
      // {
      //   test: /\.(ttf|eot|woff2?)$/,
      //   type: 'asset/resource',
      //   generator: {
      //     filename: 'fonts/[name].[contenthash:8][ext][query]',
      //   },
      // },
    ],
  },
  plugins: [
    new ImageminPlugin(),
    new MiniCssExtractPlugin({
      filename: 'styles/[name].[contenthash:8].css',
      chunkFilename: 'styles/[id].css',
    }),
    new CopyPlugin({
      patterns: [
        {
          from: 'images',
          to: 'images/[path][name].[contenthash:8][ext]',
        },
        // {
        //   from: 'fonts',
        //   to: 'fonts/[path][name].[contenthash:8][ext]',
        // },
      ],
    }),
    new PurgeCSSPlugin({
      paths: glob.sync([
        'assets/**/*.js',
        'assets/**/*.scss',
        'assets/**/*.vue',
        'templates/**/*',
      ],  { nodir: true }),

      // Needed for vue.js
      extractors: [
        {
          extractor: purgeFromVue,
          extensions: ['vue'],
        },
      ],

      safelist: [ /-(leave|enter|appear)(|-(to|from|active))$/, /^(?!(|.*?:)cursor-move).+-move$/, /^router-link(|-exact)-active$/, /data-v-.*/ ],
      // safelist: collectSafelist,
    }),
  ],
});
