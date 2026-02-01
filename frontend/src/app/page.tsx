'use client';

import Link from 'next/link';
import { Navbar } from '@/components/layout/Navbar';
import { Footer } from '@/components/layout/Footer';
import { Button } from '@/components/ui/Button';

export default function Home() {
  return (
    <div className="min-h-screen flex flex-col">
      {/* Hero Section with Stadium Background */}
      <div
        className="relative min-h-screen flex flex-col"
        style={{
          backgroundImage: 'url(/spartan-stadium.png)',
          backgroundSize: 'cover',
          backgroundPosition: 'center',
        }}
      >
        {/* Dark Overlay */}
        <div className="absolute inset-0 bg-spartan-black/70" />

        {/* Navbar */}
        <Navbar transparent />

        {/* Hero Content */}
        <div className="relative flex-1 flex items-center justify-center px-6">
          <div className="text-center max-w-3xl">
            <h1 className="text-5xl md:text-6xl font-bold text-white mb-6">
              MSU Ticket Marketplace
            </h1>
            <p className="text-xl md:text-2xl text-gray-300 mb-8">
              Buy and sell student sports tickets safely and securely
            </p>
            <div className="flex flex-col sm:flex-row gap-4 justify-center">
              <Link href="/tickets">
                <Button size="lg" className="w-full sm:w-auto">
                  Browse Tickets
                </Button>
              </Link>
              <Link href="/sell">
                <Button
                  size="lg"
                  variant="outline"
                  className="w-full sm:w-auto border-white text-white hover:bg-white hover:text-spartan-black"
                >
                  Sell Your Ticket
                </Button>
              </Link>
            </div>
          </div>
        </div>
      </div>

      {/* How It Works Section */}
      <section className="py-16 px-6 bg-white">
        <div className="max-w-6xl mx-auto">
          <h2 className="text-3xl font-bold text-center text-spartan-black mb-12">
            How It Works
          </h2>
          <div className="grid md:grid-cols-4 gap-8">
            <div className="text-center">
              <div className="w-16 h-16 bg-spartan-green rounded-full flex items-center justify-center text-white text-2xl font-bold mx-auto mb-4">
                1
              </div>
              <h3 className="font-semibold text-lg mb-2">List Your Ticket</h3>
              <p className="text-gray-600">
                Create a listing with your seat details and price
              </p>
            </div>
            <div className="text-center">
              <div className="w-16 h-16 bg-spartan-green rounded-full flex items-center justify-center text-white text-2xl font-bold mx-auto mb-4">
                2
              </div>
              <h3 className="font-semibold text-lg mb-2">Transfer to Us</h3>
              <p className="text-gray-600">
                Transfer your ticket to our Paciolan custodial account
              </p>
            </div>
            <div className="text-center">
              <div className="w-16 h-16 bg-spartan-green rounded-full flex items-center justify-center text-white text-2xl font-bold mx-auto mb-4">
                3
              </div>
              <h3 className="font-semibold text-lg mb-2">We Verify</h3>
              <p className="text-gray-600">
                Our system automatically verifies your ticket
              </p>
            </div>
            <div className="text-center">
              <div className="w-16 h-16 bg-spartan-green rounded-full flex items-center justify-center text-white text-2xl font-bold mx-auto mb-4">
                4
              </div>
              <h3 className="font-semibold text-lg mb-2">Get Paid</h3>
              <p className="text-gray-600">
                Receive payment when your ticket sells
              </p>
            </div>
          </div>
        </div>
      </section>

      <Footer />
    </div>
  );
}
