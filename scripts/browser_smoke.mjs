import { chromium } from 'playwright';
import fs from 'node:fs/promises';
import path from 'node:path';

const baseUrl = process.env.TINY_WEBUI_URL ?? 'http://127.0.0.1:8787';
const outDir = process.env.BROWSER_PROOF_DIR ?? 'browser-proof';
await fs.mkdir(outDir, { recursive: true });

const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });

try {
  await page.goto(baseUrl, { waitUntil: 'networkidle' });
  await page.getByText('Write proof file').click();
  const output = page.locator('#out');
  await output.waitFor({ state: 'visible' });
  await page.waitForFunction(() => {
    const text = document.querySelector('#out')?.textContent ?? '';
    return text.includes('"proof_path"') && text.includes('"verified": true');
  });

  const text = await output.textContent();
  if (!text) throw new Error('proof output was empty');
  const parsed = JSON.parse(text);
  if (parsed.run_id !== 'dry-run-first-smoke') throw new Error(`unexpected run_id: ${parsed.run_id}`);
  if (!parsed.proof_path.includes('.tiny-nim-agent/proofs/dry-run-first-smoke.json')) {
    throw new Error(`unexpected proof path: ${parsed.proof_path}`);
  }
  if (parsed.proof_json?.verified !== true) throw new Error('proof_json.verified was not true');

  await fs.writeFile(path.join(outDir, 'browser-proof.json'), JSON.stringify(parsed, null, 2));
  await page.screenshot({ path: path.join(outDir, 'browser-proof.png'), fullPage: true });
} finally {
  await browser.close();
}
