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
  const normalized = expression.replace(/(\d)([a-zA-Z])/g, '$1*$2');
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
  return expr
    .replace(/\*([a-zA-Z])/g, ' $1')
    .replace(/x\^(\d+)/g, 'x^{$1}')
    .replace(/\//g, '}{');
}

function toMathML(expr: string): string {
  return `<mrow>${expr.replace(/x/g, '<mi>x</mi>').replace(/([0-9]+)/g, '<mn>$1</mn>').replace(/\+/g, '<mo>+</mo>').replace(/\*/g, '<mo>&times;</mo>')}</mrow>`;
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
