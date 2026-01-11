import { Link } from 'react-router-dom'
import React from 'react'

interface PlaceholderProps {
  pageName: string
}

export default function Placeholder({ pageName }: PlaceholderProps) {
  return (
    <div className="min-h-screen bg-white">
      <nav className="flex items-center justify-between px-6 md:px-12 lg:px-20 py-6 bg-white shadow-md">
        <Link to="/" className="flex items-center gap-3">
          <div className="w-12 h-12 md:w-14 md:h-14 bg-spartan-green relative">
            <div
              className="absolute inset-0"
              style={{
                background: '#18453B',
                clipPath: 'polygon(0 0, 100% 0, 85% 100%, 0% 100%)',
                transform: 'rotate(-15deg)',
              }}
            />
          </div>
          <div className="flex flex-col">
            <span className="text-lg md:text-xl font-semibold leading-tight">Spartan</span>
            <span className="text-lg md:text-xl font-semibold leading-tight">Marketplace</span>
          </div>
        </Link>

        <div className="hidden lg:flex items-center gap-8">
          <Link to="/about" className="text-base font-normal hover:text-spartan-green transition-colors">About us</Link>
          <Link to="/services" className="text-base font-normal hover:text-spartan-green transition-colors">Services</Link>
          <Link to="/pricing" className="text-base font-normal hover:text-spartan-green transition-colors">Pricing</Link>
        </div>

        <div className="flex items-center gap-4">
          <button className="px-6 py-2 text-base font-normal hover:text-spartan-green transition-colors">Login</button>
          <button className="px-6 py-2 bg-spartan-green text-white rounded-lg hover:bg-opacity-90 transition-all">Sign Up</button>
        </div>
      </nav>

      <div className="flex items-center justify-center min-h-[calc(100vh-100px)] px-6">
        <div className="text-center max-w-2xl">
          <h1 className="text-4xl md:text-5xl font-bold text-spartan-green mb-6">{pageName}</h1>
          <p className="text-lg text-gray-600 mb-8">This page is coming soon. Continue prompting to add content to this page.</p>
          <Link to="/" className="inline-block px-9 py-5 bg-spartan-dark text-white rounded-[14px] text-xl font-normal hover:bg-opacity-90 transition-all">Back to Home</Link>
        </div>
      </div>
    </div>
  )
}
