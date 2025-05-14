'use client';

import React from 'react';
import Link from 'next/link';

const FeatureSections = () => {
  return (
    <div className="w-full max-w-5xl mx-auto px-4 py-12">
      <div className="text-center mb-12">
        <h2 className="text-3xl md:text-4xl font-light mb-6">
          Don't Just Watch Events.
          <br />
          Predict them.
        </h2>
        <div className="flex justify-center space-x-4">
          <Link
            href="/signin"
            className="px-6 py-2 rounded-full border border-zinc-800 text-white hover:border-zinc-700 text-sm transition-colors"
          >
            Sign In
          </Link>
          <Link
            href="/signup"
            className="px-6 py-2 rounded-full bg-[#FF6B00] hover:bg-[#FF6B00]/90 text-white text-sm transition-colors"
          >
            Sign Up
          </Link>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="bg-zinc-900/50 border border-zinc-800 rounded-xl p-6">
          <h3 className="text-white text-xl font-light mb-3">Trade Faster.</h3>
          <p className="text-zinc-400 text-sm">
            Cube is built for speed with snappy interactions and CPU-optimized
            rendering. Your orders are executed in a flash. We are setting a new
            standard for digital asset exchanges.
          </p>
        </div>

        <div className="bg-zinc-900/50 border border-zinc-800 rounded-xl p-6">
          <h3 className="text-white text-xl font-light mb-3">Trade Safer.</h3>
          <p className="text-zinc-400 text-sm">
            Play to earn with Cube's Telegram minigame 'Hodl High'. Tap and
            stack Blocks to redeem for cash and crypto. Climb the leaderboards
            and compete for rewards.
          </p>
          <Link
            href="#"
            className="text-[#FF6B00] text-sm hover:underline mt-3 inline-block"
          >
            Learn more →
          </Link>
        </div>

        <div className="bg-zinc-900/50 border border-zinc-800 rounded-xl p-6">
          <h3 className="text-white text-xl font-light mb-3">
            Trade Different.
          </h3>
          <p className="text-zinc-400 text-sm">
            Cube is built for speed with snappy interactions and CPU-optimized
            rendering. Your orders are executed in a flash. We are setting a new
            standard for digital asset exchanges.
          </p>
          <Link
            href="#"
            className="text-[#FF6B00] text-sm hover:underline mt-3 inline-block"
          >
            Learn more →
          </Link>
        </div>
      </div>
    </div>
  );
};

export default FeatureSections;
