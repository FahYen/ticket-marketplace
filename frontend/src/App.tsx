import React from 'react'
import { Routes, Route } from 'react-router-dom'
import Index from './pages/Index'
import NotFound from './pages/NotFound'
import Placeholder from './pages/Placeholder'

export default function App() {
  return (
    <Routes>
      <Route path="/" element={<Index />} />
      <Route path="/about" element={<Placeholder pageName="About Us" />} />
      <Route path="/services" element={<Placeholder pageName="Services" />} />
      <Route path="/pricing" element={<Placeholder pageName="Pricing" />} />
      <Route path="*" element={<NotFound />} />
    </Routes>
  )
}
