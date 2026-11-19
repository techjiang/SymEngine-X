import React, { useEffect, useState } from 'react';
import InputBar from './components/InputBar';
import ResultView from './components/ResultView';
import HistoryPanel from './components/HistoryPanel';

type Step = { label: string; value: string };
type Operation = 'simplify' | 'derive' | 'integrate' | 'factor' | 'collect';

const OPERATIONS: { key: Operation; label: string }[] = [
  { key: 'simplify', label: '化简' },
  { key: 'derive', label: '求导' },
  { key: 'integrate', label: '积分' },
  { key: 'factor', label: '因式分解' },
  { key: 'collect', label: '合并同类项' },
];

function compute(input: string, op: Operation, format: 'latex' | 'mathml'): Step[] {
  const expression = input.replace(/\s+/g, '').replace(/\*\*/g, '^');
  const steps: Step[] = [{ label: '输入', value: input }];
  const normalized = normalizeInfix(expression);
  steps.push({ label: '解析', value: normalized });

  let result = normalized;
  switch (op) {
    case 'derive':
      result = derivative(normalized);
      steps.push({ label: '求导 d/dx', value: result });
      break;
    case 'integrate':
      result = integrate(normalized);
      steps.push({ label: '积分 ∫dx', value: result });
      break;
    case 'factor':
      result = factor(normalized);
      steps.push({ label: '因式分解', value: result });
      break;
    case 'collect':
      result = collect(normalized);
      steps.push({ label: '合并同类项', value: result });
      break;
    default:
      result = normalized;
  }

  steps.push({
    label: format === 'latex' ? 'LaTeX 输出' : 'MathML 输出',
    value: format === 'latex' ? toLatex(result) : toMathML(result),
  });
  return steps;
}

function normalizeInfix(expr: string): string {
  return expr.replace(/(\d+(?:\.\d+)?)([a-zA-Z])/g, '$1*$2');
}

function derivative(expr: string): string {
  if (/^x\^(\d+)/.test(expr)) {
    const n = Number(expr.match(/^x\^(\d+)/)![1]);
    return n === 1 ? '1' : `${n}*x^${n - 1}`;
  }
  if (expr.includes('sin(')) return 'cos(x)';
  if (expr.includes('cos(')) return '-sin(x)';
  if (expr.includes('exp(')) return 'exp(x)';
  if (expr === 'x') return '1';
  return '0';
}

function integrate(expr: string): string {
  if (/^x\^(\d+)$/.test(expr)) {
    const n = Number(expr.slice(2));
    return `x^${n + 1}/${n + 1}`;
  }
  if (expr.includes('cos(')) return 'sin(x)';
  if (expr.includes('sin(')) return '-cos(x)';
  if (expr.includes('exp(')) return 'exp(x)';
  return `∫(${expr})dx`;
}

function factor(expr: string): string {
  const cleaned = expr.replace(/\s/g, '');
  const match = cleaned.match(/^x\^2([+-]\d+\*?x)?([+-]\d+)?$/);
  if (match) {
    const b = match[1] ? parseCoeff(match[1].replace('*x', '').replace('x', '')) : 0;
    const c = match[2] ? parseCoeff(match[2]) : 0;
    const roots = solveQuadratic(1, b, c);
    if (roots.length === 2 && Number.isInteger(roots[0]) && Number.isInteger(roots[1])) {
      return `(x-(${roots[0]}))*(x-(${roots[1]}))`;
    }
  }
  return cleaned;
}

function collect(expr: string): string {
  const terms: Record<string, number> = {};
  for (const part of expr.replace(/\s/g, '').split(/[+-](?![^()]*\))/)) {
    if (!part) continue;
    const m = part.match(/^(\d+\*?)?(x(?:\^\d+)?)?$/);
    if (!m) continue;
    const coeff = m[1] ? parseCoeff(m[1].replace('*', '')) : 1;
    const key = m[2] || 'const';
    terms[key] = (terms[key] || 0) + coeff;
  }
  return Object.entries(terms)
    .filter(([, v]) => v !== 0)
    .map(([k, v]) => (v === 1 && k !== 'const' ? k : `${v}${k === 'const' ? '' : '*' + k}`))
    .join('+');
}

function parseCoeff(s: string): number {
  if (!s || s === '+') return 1;
  if (s === '-') return -1;
  return Number(s) || 0;
}

function solveQuadratic(a: number, b: number, c: number): number[] {
  const disc = b * b - 4 * a * c;
  if (disc < 0) return [];
  const root = Math.sqrt(disc);
  return [(-b + root) / (2 * a), (-b - root) / (2 * a)];
}

function toLatex(expr: string): string {
  const escaped = expr
    .replace(/&/g, '\\&')
    .replace(/%/g, '\\%')
    .replace(/\$/g, '\\$')
    .replace(/#/g, '\\#')
    .replace(/_/g, '\\_')
    .replace(/\*/([a-zA-Z])/g, ' $1')
    .replace(/x\^(\d+)/g, 'x^{$1}');
  const slash = escaped.indexOf('/');
  if (slash === -1) return escaped;
  const numerator = escaped.slice(0, slash);
  const denominator = escaped.slice(slash + 1);
  return `\\frac{${numerator}}{${denominator}}`;
}

function toMathML(expr: string): string {
  const parts = expr.split(/([+*/])/).filter(Boolean);
  const body = parts.map((part) => {
    if (part === '+') return '<mo>+</mo>';
    if (part === '*') return '<mo>&times;</mo>';
    if (part === '/') return '<mo>/</mo>';
    const match = part.match(/^(\d+(?:\.\d+)?)?\*?(x(?:\^\d+)?)?$/);
    if (match) {
      const coefficient = match[1] ? `<mn>${match[1]}</mn>` : '';
      const symbol = match[2] ? renderMathMLSymbol(match[2]) : '';
      return coefficient && symbol ? `<mrow>${coefficient}<mo>&times;</mo>${symbol}</mrow>` : coefficient || symbol || `<mn>${escapeXml(part)}</mn>`;
    }
    return `<mn>${escapeXml(part)}</mn>`;
  });
  return `<mrow>${body.join('')}</mrow>`;
}

function renderMathMLSymbol(symbol: string): string {
  const match = symbol.match(/^([a-zA-Z])(?:\^(\d+))?$/);
  if (!match) return `<mi>${escapeXml(symbol)}</mi>`;
  const base = `<mi>${escapeXml(match[1])}</mi>`;
  return match[2] ? `<msup>${base}<mn>${match[2]}</mn></msup>` : base;
}

function escapeXml(value: string): string {
  return value.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
}

export default function App() {
  const [input, setInput] = useState('x^2 - 2*x + 1');
  const [steps, setSteps] = useState<Step[]>([]);
  const [history, setHistory] = useState<string[]>([]);
  const [mode, setMode] = useState<'latex' | 'mathml'>('latex');
  const [op, setOp] = useState<Operation>('simplify');

  const calculate = (value = input, operation = op, format = mode) => {
    const expression = value.trim();
    if (!expression) return;
    const computed = compute(expression, operation, format);
    const next = computed[computed.length - 1].value;
    setSteps(computed);
    setHistory((items) => [next, ...items].slice(0, 20));
  };

  useEffect(() => { calculate(); /* eslint-disable-next-line react-hooks/exhaustive-deps */ }, []);

  return (
    <main>
      <header>
        <h1>SymEngine-X <span className="badge">v1.0.0</span></h1>
        <p>符号数学计算引擎 · 在线计算器 · 5 轮迭代增强</p>
      </header>
      <section className="card">
        <InputBar value={input} onChange={setInput} onSubmit={calculate} />
        <div className="ops">
          {OPERATIONS.map((item) => (
            <button key={item.key} className={op === item.key ? 'active' : ''}
              onClick={() => { setOp(item.key); calculate(input, item.key, mode); }}>
              {item.label}
            </button>
          ))}
        </div>
        <div className="modes">
          <button className={mode === 'latex' ? 'active' : ''}
            onClick={() => { setMode('latex'); calculate(input, op, 'latex'); }}>LaTeX</button>
          <button className={mode === 'mathml' ? 'active' : ''}
            onClick={() => { setMode('mathml'); calculate(input, op, 'mathml'); }}>MathML</button>
        </div>
        <ResultView result={steps.length ? steps[steps.length - 1].value : ''} />
        {steps.length > 0 && (
          <ol className="steps">
            {steps.map((step, index) => (
              <li key={index}><span>{step.label}</span><code>{step.value}</code></li>
            ))}
          </ol>
        )}
      </section>
      <HistoryPanel items={history} />
      <footer>SymEngine-X v1.0.0 · Apache 2.0 · 科技酱</footer>
    </main>
  );
}
