#!/usr/bin/env node

const fs = require('fs');
const cheerio = require('cheerio');
const entities = require('html-entities');
const hljs = require('./build/highlight.js');

const githublink = `\
<li class="part-title">\
<a href="https://github.com/dtolnay/cxx">\
<i class="fa fa-github"></i>\
https://github.com/dtolnay/cxx\
</a>\
</li>`;

const opengraph = `\
<meta property="og:image" content="https://cxx.rs/cxx.png" />\
<meta property="og:site_name" content="CXX" />\
<meta property="og:title" content="CXX — safe interop between Rust and C++" />\
<meta name="twitter:image:src" content="https://cxx.rs/cxx.png" />\
<meta name="twitter:site" content="@davidtolnay" />\
<meta name="twitter:card" content="summary" />\
<meta name="twitter:title" content="CXX — safe interop between Rust and C++" />`;

const themejs = `\
var theme;
try { theme = localStorage.getItem('mdbook-theme'); } catch(e) {}
if (theme === null || theme === undefined) { theme = default_theme; }
const html = document.documentElement;
html.classList.remove('light')
html.classList.add(theme);
html.classList.add("js");`;

const themejsReplacement = `\
const html = document.documentElement;
html.classList.add('js');`;

const dirs = ['build'];
while (dirs.length) {
  const dir = dirs.pop();
  fs.readdirSync(dir).forEach((entry) => {
    const path = dir + '/' + entry;
    const stat = fs.statSync(path);
    if (stat.isDirectory()) {
      dirs.push(path);
      return;
    }

    if (!path.endsWith('.html')) {
      return;
    }

    const index = fs.readFileSync(path, 'utf8');
    const $ = cheerio.load(index, {
      decodeEntities: false,
      xml: { xmlMode: false },
    });

    $('head').append(opengraph);
    $('nav#sidebar ol.chapter').append(githublink);
    $('head link[href="tomorrow-night.css"]').attr('disabled', true);
    $('head link[href="ayu-highlight.css"]').attr('disabled', true);
    $('button#theme-toggle').attr('style', 'display:none');
    $('pre code').each(function () {
      const node = $(this);
      const langClass = node.attr('class').split(' ', 2)[0];
      if (!langClass.startsWith('language-')) {
        return;
      }
      const lang = langClass.replace('language-', '');
      const originalLines = node.html().split('\n');
      const boring = originalLines.map((line) =>
        line.includes('<span class="boring">'),
      );
      const ellipsis = originalLines.map((line) => line.includes('// ...'));
      const target = entities.decode(node.text());
      const highlightedLines = hljs.highlight(lang, target).value.split('\n');
      const result = highlightedLines
        .map(function (line, i) {
          if (boring[i]) {
            line = '<span class="boring">' + line;
          } else if (ellipsis[i]) {
            line = '<span class="ellipsis">' + line;
          }
          if (i > 0 && (boring[i - 1] || ellipsis[i - 1])) {
            line = '</span>' + line;
          }
          if (i + 1 === highlightedLines.length && (boring[i] || ellipsis[i])) {
            line = line + '</span>';
          }
          return line;
        })
        .join('\n');
      node.text(result);
      node.removeClass(langClass);
      if (!node.hasClass('focuscomment')) {
        node.addClass('hidelines');
        node.addClass('hide-boring');
      }
    });
    $('code').each(function () {
      $(this).addClass('hljs');
    });

    var foundScript = false;
    $('body script').each(function () {
      const node = $(this);
      if (node.text().replace(/\s/g, '') === themejs.replace(/\s/g, '')) {
        node.text(themejsReplacement);
        foundScript = true;
      }
    });
    const pathsWithoutScript = [
      'build/toc.html',
      'build/build/index.html',
      'build/binding/index.html',
    ];
    if (!foundScript && !pathsWithoutScript.includes(path)) {
      throw new Error(`theme script not found in ${path}`);
    }

    const out = $.html();
    fs.writeFileSync(path, out);
  });
}

fs.copyFileSync('build/highlight.css', 'build/tomorrow-night.css');
fs.copyFileSync('build/highlight.css', 'build/ayu-highlight.css');

var bookjs = fs.readFileSync('build/book.js', 'utf8');
bookjs = bookjs
  .replace('set_theme(theme, false);', '')
  .replace(
    'document.querySelectorAll("code.hljs")',
    'document.querySelectorAll("code.hidelines")',
  );
fs.writeFileSync('build/book.js', bookjs);
