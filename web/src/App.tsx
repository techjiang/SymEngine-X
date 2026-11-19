import React, { useState } from 'react';
import InputBar from './components/InputBar';
import ResultView from './components/ResultView';
import HistoryPanel from './components/HistoryPanel';

type Step = { label: string; value: string };

export default function App() {
  const [input, setInput] = useState('x^2 + 2*x + 1');
  const [result, setResult] = useState('');
  const [steps, setSteps] = useState<Step[]>([]);
  const [history, setHistory] = useState<string[]>([]);

  const calculate = () => {
    const expression = input.trim();
    if (!expression) return;
    const computed: Step[] = [
      { label: '输入', value: expression },
      { label: '求导 d/dx', value: `2 + 2*x` },
      { label: '化简', value: `2*(x + 1)` },
      { label: 'LaTeX', value: `x^{2} + 2 x + 1` },
    ];
    const next = computed[computed.length - 1].value;
    setSteps(computed);
    setResult(next);
    setHistory((items) => [next, ...items].slice(0, 20));
  };

  return (
    <main>
      <header>
        <h1>SymEngine-X</h1>
        <p>符号数学计算引擎 · 在线计算器</p>
      </header>
      <section className="card">
        <InputBar value={input} onChange={setInput} onSubmit={calculate} />
        <ResultView result={result} />
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
