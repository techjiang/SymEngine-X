import React from 'react';
export default function ResultView({ result }: { result: string }) {
  return <pre aria-label="result">{result}</pre>;
}
