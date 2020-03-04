/* eslint-disable @typescript-eslint/no-var-requires */
const path = require('path');
const webpack = require('webpack');
const ForkTsCheckerNotifierWebpackPlugin = require('fork-ts-checker-notifier-webpack-plugin');
const ForkTsCheckerWebpackPlugin = require('fork-ts-checker-webpack-plugin');
const ScriptExtHtmlWebpackPlugin = require('script-ext-html-webpack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
    mode: "none", //maybe a tiny speedup - but use DefinePlugin manually
    context: process.cwd(), // to automatically find tsconfig.json
    entry: "./typescript/index.ts",
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: '[name].js',
        publicPath: "/"
    },
    plugins: [
        new webpack.DefinePlugin({
            'process.env.NODE_ENV': JSON.stringify("development")
        }),
        new ForkTsCheckerWebpackPlugin({
            eslint: false
        }),
        new ForkTsCheckerNotifierWebpackPlugin({ title: 'TypeScript', excludeWarnings: false }),
        new HtmlWebpackPlugin({
            inject: true,
            template: 'typescript/index.html'
        }),
        new ScriptExtHtmlWebpackPlugin({
            defaultAttribute: 'defer'
        }),
        //new webpack.HotModuleReplacementPlugin()
    ],
    module: {
        rules: [
            {
                test: /\.css$/i,
                use: [ 'lit-css-loader'],
            },
            {
                test: /.ts$/,
                use: [
                    { loader: 'ts-loader', options: { transpileOnly: true } }
                ],
                exclude: [
                    path.resolve(__dirname, "typescript/tests/**/*"),
                ]

            },
        ]
    },
    resolve: {
        extensions: [".ts", ".js", ".css"],
        alias: {
            "@components": path.resolve(__dirname, "typescript/components"),
            "@styles": path.resolve(__dirname, "typescript/components/styles"),
            "@utils": path.resolve(__dirname, "typescript/utils"),
            "@settings": path.resolve(__dirname, "typescript/settings"),
            "@events": path.resolve(__dirname, "typescript/events"),
        }
    },
    devtool: 'inline-source-map',
    devServer: {
        contentBase: path.resolve(__dirname, './_static'),
        compress: true,
        clientLogLevel: 'warning',
        open: true,
        historyApiFallback: true,
        stats: 'errors-only',
        watchOptions: {
            ignored: ['node_modules', 'target', 'pkg', '**/*.rs']
        },
        watchContentBase: true,
    }
};