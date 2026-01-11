import React from 'react'
import { Link } from 'react-router-dom'

export default function NotFound() {
  return (
    <div className="min-h-screen flex items-center justify-center bg-white px-6">
      <div className="text-center">
        <h1 className="text-5xl font-bold text-spartan-green mb-4">404</h1>
        <p className="text-lg text-gray-600 mb-8">Page not found.</p>
        <Link to="/" className="inline-block px-6 py-3 bg-spartan-dark text-white rounded">Back Home</Link>
      </div>
    </div>
  )
}
