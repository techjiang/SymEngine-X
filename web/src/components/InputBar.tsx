import React from 'react';

type Props = { value: string; onChange: (value: string) => void; onSubmit: () => void };

export default function InputBar({ value, onChange, onSubmit }: Props) {
  return (
    <div>
      <input value={value} onChange={(event) => onChange(event.target.value)} aria-label="expression" />
      <button onClick={onSubmit}>计算</button>
    </div>
  );
}
