import React, { useState } from 'react';
import InputBar from './components/InputBar';
import ResultView from './components/ResultView';
import HistoryPanel from './components/HistoryPanel';

export default function App() {
  const [input, setInput] = useState('x^2 + 2*x + 1');
  const [result, setResult] = useState('');
  const [history, setHistory] = useState<string[]>([]);

  const calculate = () => {
    const next = `简化结果骨架：${input}`;
    setResult(next);
    setHistory((items) => [next, ...items].slice(0, 20));
  };

  return (
    <main>
      <h1>SymEngine-X Web Calculator</h1>
      <InputBar value={input} onChange={setInput} onSubmit={calculate} />
      <ResultView result={result} />
      <HistoryPanel items={history} />
    </main>
  );
}
