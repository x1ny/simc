"use client";

import { useEffect, useState } from 'react';
import  simc, { WebSimc } from 'simc/simc';

export default function Home() {

  const [str, setStr] = useState('')

  useEffect(() => {
    simc()
      .then(() => {
        setStr(WebSimc.say_hi())
      })
  }, [])

  return (
    <div>{str}</div>
  );
}
