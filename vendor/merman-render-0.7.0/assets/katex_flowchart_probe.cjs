const fs = require('fs');
const path = require('path');
const url = require('url');
const { createRequire } = require('module');

const requireFromCwd = createRequire(path.resolve(process.cwd(), 'package.json'));
const katex = requireFromCwd('katex');

const lineBreakRegex = /<br\s*\/?>/gi;
const katexRegex = /\$\$(.*)\$\$/g;

function hasKatex(text) {
  return String(text ?? '').includes('$$');
}

function katexCssPath() {
  try {
    return requireFromCwd.resolve('katex/dist/katex.css');
  } catch {
    return null;
  }
}

async function addKatexCss(page) {
  const cssPath = katexCssPath();
  if (!cssPath) {
    return;
  }
  await page.addStyleTag({ path: cssPath });
}

function renderKatexHtml(text, config) {
  const input = String(text ?? '');
  if (!hasKatex(input)) {
    return input;
  }

  const output =
    config && config.forceLegacyMathML ? 'htmlAndMathml' : 'mathml';

  return input
    .split(lineBreakRegex)
    .map((line) =>
      hasKatex(line)
        ? `<div style="display: flex; align-items: center; justify-content: center; white-space: nowrap;">${line}</div>`
        : `<div>${line}</div>`
    )
    .join('')
    .replace(katexRegex, (_, content) =>
      katex
        .renderToString(content, {
          throwOnError: true,
          displayMode: true,
          output,
        })
        .replace(/\n/g, ' ')
        .replace(/<annotation.*<\/annotation>/g, '')
    );
}

function browserExecutableCandidates() {
  const candidates = [];
  if (process.env.PUPPETEER_EXECUTABLE_PATH) {
    candidates.push(process.env.PUPPETEER_EXECUTABLE_PATH);
  }

  if (process.platform === 'win32') {
    candidates.push(
      'C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe',
      'C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome.exe',
      'C:\\Program Files\\Microsoft\\Edge\\Application\\msedge.exe',
      'C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe'
    );
  } else if (process.platform === 'darwin') {
    candidates.push(
      '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',
      '/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge'
    );
  } else {
    candidates.push(
      '/usr/bin/google-chrome',
      '/usr/bin/google-chrome-stable',
      '/usr/bin/chromium',
      '/usr/bin/chromium-browser',
      '/snap/bin/chromium'
    );
  }

  return [...new Set(candidates)].filter((candidate) => fs.existsSync(candidate));
}

async function launchBrowser(puppeteer, launchOptions) {
  const candidates = browserExecutableCandidates();
  const shouldTryDefault = !process.env.PUPPETEER_EXECUTABLE_PATH;
  let firstError;

  if (shouldTryDefault) {
    try {
      return await puppeteer.launch(launchOptions);
    } catch (error) {
      firstError = error;
    }
  }

  for (const executablePath of candidates) {
    try {
      return await puppeteer.launch({ ...launchOptions, executablePath });
    } catch (error) {
      firstError ??= error;
    }
  }

  throw firstError || new Error('no browser executable candidate found');
}

async function measureHtml(html, styleCss, maxWidthPx) {
  const puppeteer = requireFromCwd('puppeteer');
  const mermaidCliIndexHtml = path.join(
    process.cwd(),
    'node_modules',
    '@mermaid-js',
    'mermaid-cli',
    'dist',
    'index.html'
  );
  const browser = await launchBrowser(puppeteer, {
    headless: 'shell',
    args: [
      '--no-sandbox',
      '--disable-setuid-sandbox',
      '--allow-file-access-from-files',
      '--force-device-scale-factor=1',
    ],
  });
  try {
    const page = await browser.newPage();
    await page.setViewport({
      width: 1200,
      height: 800,
      deviceScaleFactor: 1,
    });
    await page.goto(url.pathToFileURL(mermaidCliIndexHtml).href);
    await addKatexCss(page);
    return await page.evaluate(
      (payload) => {
        const SVG_NS = 'http://www.w3.org/2000/svg';
        const XHTML_NS = 'http://www.w3.org/1999/xhtml';
        const host = document.getElementById('container') || document.body;
        host.innerHTML = '';

        const svg = document.createElementNS(SVG_NS, 'svg');
        svg.setAttribute('xmlns', SVG_NS);
        svg.setAttribute('width', `${payload.maxWidthPx * 10}`);
        svg.setAttribute('height', `${payload.maxWidthPx * 10}`);
        svg.style.position = 'absolute';
        svg.style.top = '0';
        svg.style.left = '0';
        svg.style.visibility = 'hidden';

        const fo = document.createElementNS(SVG_NS, 'foreignObject');
        fo.setAttribute('width', `${payload.maxWidthPx * 10}`);
        fo.setAttribute('height', `${payload.maxWidthPx * 10}`);
        svg.appendChild(fo);

        const div = document.createElementNS(XHTML_NS, 'div');
        if (payload.styleCss) {
          div.setAttribute('style', payload.styleCss);
        }
        div.style.display = 'table-cell';
        div.style.whiteSpace = 'nowrap';
        div.style.lineHeight = '1.5';
        div.style.maxWidth = `${payload.maxWidthPx}px`;
        div.style.textAlign = 'center';
        div.setAttribute('xmlns', XHTML_NS);
        fo.appendChild(div);

        const sanitizeRenderedMathHtml = (html) => {
          const cleaned =
            typeof DOMPurify !== 'undefined'
              ? DOMPurify.sanitize(html, { FORBID_TAGS: ['style'] }).toString()
              : html;
          return cleaned.replace(/<\/?semantics>/g, '');
        };
        const sanitizedHtml = sanitizeRenderedMathHtml(payload.html);

        const span = document.createElementNS(XHTML_NS, 'span');
        span.className = 'nodeLabel';
        if (payload.styleCss) {
          span.setAttribute('style', payload.styleCss);
        }
        span.innerHTML = sanitizedHtml;
        div.appendChild(span);
        host.appendChild(svg);

        let bbox = div.getBoundingClientRect();
        fo.setAttribute('width', `${bbox.width}`);
        fo.setAttribute('height', `${bbox.height}`);
        if (bbox.width === payload.maxWidthPx) {
          div.style.display = 'table';
          div.style.whiteSpace = 'break-spaces';
          div.style.width = `${payload.maxWidthPx}px`;
          bbox = div.getBoundingClientRect();
          fo.setAttribute('width', `${bbox.width}`);
          fo.setAttribute('height', `${bbox.height}`);
        }

        return {
          html: sanitizedHtml,
          width: bbox.width,
          height: bbox.height,
        };
      },
      {
        html,
        styleCss,
        maxWidthPx,
      }
    );
  } finally {
    await browser.close();
  }
}

async function measureSequenceHtml(html) {
  const puppeteer = requireFromCwd('puppeteer');
  const browser = await launchBrowser(puppeteer, {
    headless: 'shell',
    args: [
      '--no-sandbox',
      '--disable-setuid-sandbox',
      '--allow-file-access-from-files',
      '--force-device-scale-factor=1',
    ],
  });
  try {
    const page = await browser.newPage();
    await page.setViewport({
      width: 1200,
      height: 800,
      deviceScaleFactor: 1,
    });
    await page.setContent('<!doctype html><html><body></body></html>');
    await addKatexCss(page);
    return await page.evaluate((payload) => {
      const SVG_NS = 'http://www.w3.org/2000/svg';
      const XHTML_NS = 'http://www.w3.org/1999/xhtml';

      const svg = document.createElementNS(SVG_NS, 'svg');
      svg.setAttribute('xmlns', SVG_NS);
      svg.setAttribute('width', '1200');
      svg.setAttribute('height', '800');

      const foreignObject = document.createElementNS(SVG_NS, 'foreignObject');
      svg.appendChild(foreignObject);

      const div = document.createElementNS(XHTML_NS, 'div');
      div.setAttribute('style', 'width: fit-content;');
      div.setAttribute('xmlns', XHTML_NS);
      div.innerHTML = payload.html;
      foreignObject.appendChild(div);

      document.body.appendChild(svg);
      const bbox = div.getBoundingClientRect();
      svg.remove();

      return {
        html: payload.html,
        width: bbox.width,
        height: bbox.height,
      };
    }, { html });
  } finally {
    await browser.close();
  }
}

async function main() {
  const mode = process.argv[2];
  const raw = fs.readFileSync(0, 'utf8');
  const payload = raw ? JSON.parse(raw) : {};
  const html = renderKatexHtml(payload.text, payload.config || {});

  if (mode === 'render') {
    process.stdout.write(JSON.stringify({ html }));
    return;
  }

  if (mode === 'probe') {
    const result = await measureHtml(
      html,
      payload.styleCss || '',
      Number.isFinite(payload.maxWidthPx) ? payload.maxWidthPx : 200
    );
    process.stdout.write(JSON.stringify({ html, ...result }));
    return;
  }

  if (mode === 'probe-sequence') {
    const result = await measureSequenceHtml(html);
    process.stdout.write(JSON.stringify({ html, ...result }));
    return;
  }

  throw new Error(`unknown mode: ${mode}`);
}

main().catch((error) => {
  console.error(error && error.stack ? error.stack : String(error));
  process.exit(1);
});
