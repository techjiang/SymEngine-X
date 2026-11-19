import React from 'react';
export default function HistoryPanel({ items }: { items: string[] }) {
  return <ul aria-label="history">{items.map((item, index) => <li key={index}>{item}</li>)}</ul>;
}
