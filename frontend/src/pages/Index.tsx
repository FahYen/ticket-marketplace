import { Link } from 'react-router-dom'
import React, { useState } from 'react'
import stadiumImg from '../assets/images/spartan_stadium.webp'
import GameCard from '../components/GameCard'

export default function Index() {
  const [buyersExpanded, setBuyersExpanded] = useState([true, false, false, false]);
  const [sellersExpanded, setSellersExpanded] = useState([true, false, false, false]);

  const toggleBuyer = (index: number) => {
    setBuyersExpanded(prev => prev.map((val, i) => i === index ? !val : val));
  };

  const toggleSeller = (index: number) => {
    setSellersExpanded(prev => prev.map((val, i) => i === index ? !val : val));
  };

  return (
    <div className="min-h-screen bg-white relative overflow-hidden">
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
          <Link to="/about" className="text-base font-normal hover:text-spartan-green transition-colors">
            About us
          </Link>
          <Link to="/services" className="text-base font-normal hover:text-spartan-green transition-colors">
            Services
          </Link>
          <Link to="/pricing" className="text-base font-normal hover:text-spartan-green transition-colors">
            Pricing
          </Link>
        </div>

        <div className="flex items-center gap-4">
          <button className="px-6 py-2 text-base font-normal hover:text-spartan-green transition-colors">Login</button>
          <button className="px-6 py-2 bg-spartan-green text-white rounded-lg hover:bg-opacity-90 transition-all">Sign Up</button>
        </div>
      </nav>

      <div className="relative h-[calc(100vh-100px)] min-h-[600px] overflow-hidden shadow-lg">
        <img
            src={stadiumImg}
            alt="Spartan Stadium"
            className="absolute inset-0 w-full h-full object-cover"
        />

        <div className="absolute inset-0 flex items-center justify-center px-6 transform -translate-y-[165px]">
              <div className="max-w-3xl text-center">
            <h1 className="text-4xl md:text-5xl lg:text-[55px] font-bold italic leading-tight lg:leading-[69px] text-spartan-green mb-6">
              Built by Spartans.
              <br />
              Traded by Spartans.
            </h1>

            <p className="text-base md:text-lg leading-7 lg:leading-[28px] text-black max-w-lg mx-auto mb-12">
              Our digital marketplace agency helps student trade ticket risk-free. With only 5% + $3 of seller fee and $0 buyer fee, you can Buy & Sell tickets with students you trust.
            </p>

            <div className="flex flex-col sm:flex-row items-center justify-center gap-6">
              <button className="w-full sm:w-auto px-9 py-5 bg-spartan-dark text-white rounded-[14px] text-xl font-normal leading-7 hover:bg-opacity-90 transition-all font-space-grotesk">
                Buy a Ticket
              </button>
              <button className="w-full sm:w-auto px-9 py-5 bg-spartan-dark text-white rounded-[14px] text-xl font-normal leading-7 hover:bg-opacity-90 transition-all font-space-grotesk">
                Sell a Ticket
              </button>
            </div>
          </div>
        </div>
      </div>
    
    {/* Upcoming Spartan Games Section */}
      <div className="py-12 md:py-16 px-6 bg-white">
        <div className="relative z-10 max-w-7xl mx-auto">
          {/* Heading & Subheading */}
          <div className="flex flex-col items-center gap-4 mb-12">
            {/* Green Heading */}
            <div className="inline-flex flex-col items-center">
              <div className="px-6 md:px-8 py-3 bg-spartan-green rounded-lg">
                <h2 className="text-3xl md:text-4xl lg:text-[40px] font-medium italic text-white text-center leading-normal">
                  Upcoming Spartan Games
                </h2>
              </div>
            </div>
            
            {/* Subheading */}
            <p className="text-lg md:text-xl text-center text-black leading-normal max-w-xl">
              All prices shown include fees.<br />
              What you see is what you pay.
            </p>
          </div>

          {/* Game Cards */}
          <div className="flex flex-col gap-10">
            <GameCard
              opponent="Western Michigan University"
              opponentAbbr="(WMU)"
              opponentColor="#532E1F"
              days="03"
              hours="23"
              minutes="38"
              price="29"
            />

            <GameCard
              opponent="Boston College"
              opponentColor="#8C2232"
              days="03"
              hours="23"
              minutes="38"
              price="29"
            />

            <GameCard
              opponent="Youngstown St."
              opponentColor="#C8333B"
              days="03"
              hours="23"
              minutes="38"
              price="29"
            />

            <GameCard
              opponent="University of Southern California"
              opponentAbbr="(USC)"
              opponentColor="#990000"
              days="03"
              hours="23"
              minutes="38"
              price="29"
            />

            <GameCard
              opponent="Nebraska"
              opponentColor="#D00000"
              days="03"
              hours="23"
              minutes="38"
              price="29"
            />

            {/* And More... Card */}
            <div className="w-full max-w-7xl mx-auto px-4 sm:px-6 lg:px-12">
              <div className="relative w-full rounded-[45px] border border-spartan-dark bg-[#F3F3F3] overflow-hidden"
                   style={{ boxShadow: '0 5px 0 0 #191A23' }}>
                <div className="flex items-center p-10">
                  <div className="flex items-center gap-4">
                    <div className="w-10 h-10 lg:w-[41px] lg:h-[41px] rounded-full bg-spartan-dark flex items-center justify-center flex-shrink-0">
                      <svg width="20" height="18" viewBox="0 0 20 18" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M0.75 4.20098C0.0326046 4.61518 -0.213226 5.53256 0.201046 6.25C0.615318 6.96744 1.53256 7.21327 2.25 6.79902L1.5 5.5L0.75 4.20098ZM20.2694 -4.11176C20.4839 -4.91196 20.009 -5.73449 19.2088 -5.94893L6.16884 -9.44293C5.36864 -9.65737 4.54612 -9.18246 4.33168 -8.38226C4.11724 -7.58206 4.59215 -6.75954 5.39235 -6.5451L16.9834 -3.43928L13.8776 8.15177C13.6632 8.95197 14.1381 9.77449 14.9383 9.98893C15.7385 10.2034 16.561 9.72845 16.7754 8.92825L20.2694 -4.11176ZM1.5 5.5L2.25 6.79902L19.5706 -3.20098L18.8206 -4.5L18.0706 -5.79902L0.75 4.20098L1.5 5.5Z" fill="#B9FF66"/>
                      </svg>
                    </div>
                    <span className="text-black font-space-grotesk text-lg lg:text-xl font-bold underline">
                      And More...
                    </span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* How It Works Section */}
      <div className="py-12 md:py-16 px-6 bg-white">
        <div className="max-w-7xl mx-auto text-center">
          <div className="inline-flex flex-col items-center gap-4 mb-16">
            <div className="px-6 py-2.5 bg-spartan-green rounded-lg">
              <h2 className="text-4xl md:text-5xl lg:text-[40px] font-inter italic font-medium text-white">
                How It Works
              </h2>
            </div>
            <p className="text-lg md:text-xl text-black font-space-grotesk max-w-xl">
              A simple, student-verified way to buy and sell tickets
            </p>
          </div>

          {/* Buyers and Sellers Steps Side by Side */}
          <div className="flex flex-col lg:flex-row gap-10 lg:gap-20">
            {/* Buyers Steps */}
            <div className="flex-1 flex flex-col items-center gap-10">
              <h3 className="text-4xl font-inter font-bold text-black">-Buyers-</h3>
              <div className="w-full max-w-4xl flex flex-col gap-10">
                {/* Step 1 */}
                <div className="w-full p-10 bg-[#CDFF93] rounded-[45px] border border-spartan-dark" style={{ boxShadow: '0 5px 0 0 #191A23' }}>
                  <div className="flex justify-between items-center">
                    <div className="flex items-center gap-6">
                      <span className="text-6xl font-space-grotesk font-medium text-black">01</span>
                      <span className="text-3xl font-space-grotesk font-medium text-black">Choose Your Game</span>
                    </div>
                    <div className="w-14 h-14 bg-[#F3F3F3] border border-spartan-dark rounded-full flex items-center justify-center cursor-pointer" onClick={() => toggleBuyer(0)}>
                      {buyersExpanded[0] ? <div className="w-5 h-1 bg-black"></div> : <div className="relative"><div className="absolute w-6 h-1 bg-spartan-dark top-1/2 transform -translate-y-1/2"></div><div className="absolute w-1 h-6 bg-spartan-dark left-1/2 transform -translate-x-1/2"></div></div>}
                    </div>
                  </div>
                  {buyersExpanded[0] && (
                    <>
                      <hr className="border-black my-8" />
                      <p className="text-lg font-space-grotesk font-normal text-black">Browse upcoming games and choose the one you want to attend.</p>
                    </>
                  )}
                </div>

                {/* Step 2 */}
                <div className="w-full p-10 bg-[#F3F3F3] rounded-[45px] border border-spartan-dark" style={{ boxShadow: '0 5px 0 0 #191A23' }}>
                  <div className="flex justify-between items-center">
                    <div className="flex items-center gap-6">
                      <span className="text-6xl font-space-grotesk font-medium text-black">02</span>
                      <span className="text-3xl font-space-grotesk font-medium text-black">Pay Securely</span>
                    </div>
                    <div className="w-14 h-14 bg-[#F3F3F3] border border-spartan-dark rounded-full flex items-center justify-center cursor-pointer" onClick={() => toggleBuyer(1)}>
                      {buyersExpanded[1] ? <div className="w-5 h-1 bg-black"></div> : <div className="relative"><div className="absolute w-6 h-1 bg-spartan-dark top-1/2 transform -translate-y-1/2"></div><div className="absolute w-1 h-6 bg-spartan-dark left-1/2 transform -translate-x-1/2"></div></div>}
                    </div>
                  </div>
                  {buyersExpanded[1] && (
                    <>
                      <hr className="border-black my-8" />
                      <p className="text-lg font-space-grotesk font-normal text-black">Complete your payment through our secure platform.</p>
                    </>
                  )}
                </div>

                {/* Step 3 */}
                <div className="w-full p-10 bg-[#F3F3F3] rounded-[45px] border border-spartan-dark" style={{ boxShadow: '0 5px 0 0 #191A23' }}>
                  <div className="flex justify-between items-center">
                    <div className="flex items-center gap-6">
                      <span className="text-6xl font-space-grotesk font-medium text-black">03</span>
                      <span className="text-3xl font-space-grotesk font-medium text-black">Receive Your Ticket</span>
                    </div>
                    <div className="w-14 h-14 bg-[#F3F3F3] border border-spartan-dark rounded-full flex items-center justify-center cursor-pointer" onClick={() => toggleBuyer(2)}>
                      {buyersExpanded[2] ? <div className="w-5 h-1 bg-black"></div> : <div className="relative"><div className="absolute w-6 h-1 bg-spartan-dark top-1/2 transform -translate-y-1/2"></div><div className="absolute w-1 h-6 bg-spartan-dark left-1/2 transform -translate-x-1/2"></div></div>}
                    </div>
                  </div>
                  {buyersExpanded[2] && (
                    <>
                      <hr className="border-black my-8" />
                      <p className="text-lg font-space-grotesk font-normal text-black">Get your digital ticket instantly after payment.</p>
                    </>
                  )}
                </div>

                {/* Step 4 */}
                <div className="w-full p-10 bg-[#F3F3F3] rounded-[45px] border border-spartan-dark" style={{ boxShadow: '0 5px 0 0 #191A23' }}>
                  <div className="flex justify-between items-center">
                    <div className="flex items-center gap-6">
                      <span className="text-6xl font-space-grotesk font-medium text-black">04</span>
                      <span className="text-3xl font-space-grotesk font-medium text-black">Accept Transfer</span>
                    </div>
                    <div className="w-14 h-14 bg-[#F3F3F3] border border-spartan-dark rounded-full flex items-center justify-center cursor-pointer" onClick={() => toggleBuyer(3)}>
                      {buyersExpanded[3] ? <div className="w-5 h-1 bg-black"></div> : <div className="relative"><div className="absolute w-6 h-1 bg-spartan-dark top-1/2 transform -translate-y-1/2"></div><div className="absolute w-1 h-6 bg-spartan-dark left-1/2 transform -translate-x-1/2"></div></div>}
                    </div>
                  </div>
                  {buyersExpanded[3] && (
                    <>
                      <hr className="border-black my-8" />
                      <p className="text-lg font-space-grotesk font-normal text-black">Confirm the ticket transfer to your account.</p>
                    </>
                  )}
                </div>
              </div>
            </div>

            {/* Sellers Steps */}
            <div className="flex-1 flex flex-col items-center gap-10">
              <h3 className="text-4xl font-inter font-bold text-black">-Sellers-</h3>
              <div className="w-full max-w-4xl flex flex-col gap-10">
                {/* Step 1 */}
                <div className="w-full p-10 bg-[#94BEFF] rounded-[45px] border border-spartan-dark" style={{ boxShadow: '0 5px 0 0 #191A23' }}>
                  <div className="flex justify-between items-center">
                    <div className="flex items-center gap-6">
                      <span className="text-6xl font-space-grotesk font-medium text-black">01</span>
                      <span className="text-3xl font-space-grotesk font-medium text-black">List your Ticket</span>
                    </div>
                    <div className="w-14 h-14 bg-[#F3F3F3] border border-spartan-dark rounded-full flex items-center justify-center cursor-pointer" onClick={() => toggleSeller(0)}>
                      {sellersExpanded[0] ? <div className="w-5 h-1 bg-black"></div> : <div className="relative"><div className="absolute w-6 h-1 bg-spartan-dark top-1/2 transform -translate-y-1/2"></div><div className="absolute w-1 h-6 bg-spartan-dark left-1/2 transform -translate-x-1/2"></div></div>}
                    </div>
                  </div>
                  {sellersExpanded[0] && (
                    <>
                      <hr className="border-black my-8" />
                      <p className="text-lg font-space-grotesk font-normal text-black">List your ticket for a game you can’t attend.</p>
                    </>
                  )}
                </div>

                {/* Step 2 */}
                <div className="w-full p-10 bg-[#F3F3F3] rounded-[45px] border border-spartan-dark" style={{ boxShadow: '0 5px 0 0 #191A23' }}>
                  <div className="flex justify-between items-center">
                    <div className="flex items-center gap-6">
                      <span className="text-6xl font-space-grotesk font-medium text-black">02</span>
                      <span className="text-3xl font-space-grotesk font-medium text-black">Transfer ticket</span>
                    </div>
                    <div className="w-14 h-14 bg-[#F3F3F3] border border-spartan-dark rounded-full flex items-center justify-center cursor-pointer" onClick={() => toggleSeller(1)}>
                      {sellersExpanded[1] ? <div className="w-5 h-1 bg-black"></div> : <div className="relative"><div className="absolute w-6 h-1 bg-spartan-dark top-1/2 transform -translate-y-1/2"></div><div className="absolute w-1 h-6 bg-spartan-dark left-1/2 transform -translate-x-1/2"></div></div>}
                    </div>
                  </div>
                  {sellersExpanded[1] && (
                    <>
                      <hr className="border-black my-8" />
                      <p className="text-lg font-space-grotesk font-normal text-black">Initiate the ticket transfer to the buyer.</p>
                    </>
                  )}
                </div>

                {/* Step 3 */}
                <div className="w-full p-10 bg-[#F3F3F3] rounded-[45px] border border-spartan-dark" style={{ boxShadow: '0 5px 0 0 #191A23' }}>
                  <div className="flex justify-between items-center">
                    <div className="flex items-center gap-6">
                      <span className="text-6xl font-space-grotesk font-medium text-black">03</span>
                      <span className="text-3xl font-space-grotesk font-medium text-black">Secure Auto-Transfer</span>
                    </div>
                    <div className="w-14 h-14 bg-[#F3F3F3] border border-spartan-dark rounded-full flex items-center justify-center cursor-pointer" onClick={() => toggleSeller(2)}>
                      {sellersExpanded[2] ? <div className="w-5 h-1 bg-black"></div> : <div className="relative"><div className="absolute w-6 h-1 bg-spartan-dark top-1/2 transform -translate-y-1/2"></div><div className="absolute w-1 h-6 bg-spartan-dark left-1/2 transform -translate-x-1/2"></div></div>}
                    </div>
                  </div>
                  {sellersExpanded[2] && (
                    <>
                      <hr className="border-black my-8" />
                      <p className="text-lg font-space-grotesk font-normal text-black">Our system handles the secure transfer automatically.</p>
                    </>
                  )}
                </div>

                {/* Step 4 */}
                <div className="w-full p-10 bg-[#F3F3F3] rounded-[45px] border border-spartan-dark" style={{ boxShadow: '0 5px 0 0 #191A23' }}>
                  <div className="flex justify-between items-center">
                    <div className="flex items-center gap-6">
                      <span className="text-6xl font-space-grotesk font-medium text-black">04</span>
                      <span className="text-3xl font-space-grotesk font-medium text-black">Get Paid!</span>
                    </div>
                    <div className="w-14 h-14 bg-[#F3F3F3] border border-spartan-dark rounded-full flex items-center justify-center cursor-pointer" onClick={() => toggleSeller(3)}>
                      {sellersExpanded[3] ? <div className="w-5 h-1 bg-black"></div> : <div className="relative"><div className="absolute w-6 h-1 bg-spartan-dark top-1/2 transform -translate-y-1/2"></div><div className="absolute w-1 h-6 bg-spartan-dark left-1/2 transform -translate-x-1/2"></div></div>}
                    </div>
                  </div>
                  {sellersExpanded[3] && (
                    <>
                      <hr className="border-black my-8" />
                      <p className="text-lg font-space-grotesk font-normal text-black">Receive your payment once the transfer is complete.</p>
                    </>
                  )}
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

            {/* Contact Us Section */}
      <section className="py-12 md:py-16 px-6 bg-white">
        <div className="max-w-7xl mx-auto">
          {/* Header row */}
          <div className="flex flex-col md:flex-row md:items-center md:gap-10 gap-4">
            <div className="inline-flex flex-col">
              <div className="bg-[#B9FF66] rounded-[7px] px-[7px] py-1 inline-flex">
                <h2 className="text-black text-[32px] md:text-[40px] font-space-grotesk font-medium">
                  Contact Us
                </h2>
              </div>
            </div>

            <p className="text-black text-[18px] font-space-grotesk font-normal">
              We&apos;d love to hear from you.
            </p>
          </div>

          {/* Card */}
          <div className="mt-10">
            <div
              className="w-full bg-[#F3F3F3] rounded-[45px] relative overflow-hidden border border-spartan-dark"
              style={{ boxShadow: "0 5px 0 0 #191A23" }}
            >
              {/* Decorative gradient shapes */}
              <div
                className="hidden md:block absolute right-[140px] bottom-[80px] w-[305px] h-[158px] border border-spartan-dark"
                style={{
                  transform: "rotate(-20deg)",
                  transformOrigin: "top left",
                  background: "linear-gradient(180deg, white 0%, #999999 100%)",
                }}
              />
              <div
                className="hidden md:block absolute right-[40px] top-[20px] w-[252px] h-[210px] border border-spartan-dark"
                style={{
                  transform: "rotate(40deg)",
                  transformOrigin: "top left",
                  background: "linear-gradient(180deg, #18453B 0%, #0B9A6D 100%)",
                }}
              />

              {/* Form content */}
              <div className="relative p-8 md:p-12">
                <form className="max-w-[556px] space-y-6">
                  {/* Name */}
                  <div className="space-y-2">
                    <label className="block text-black text-[16px] font-space-grotesk leading-7">
                      Name
                    </label>
                    <input
                      type="text"
                      placeholder="Name"
                      className="w-full bg-white rounded-[14px] border border-black px-[30px] py-[18px] text-[18px] font-space-grotesk outline-none"
                    />
                  </div>

                  {/* Email */}
                  <div className="space-y-2">
                    <label className="block text-black text-[16px] font-space-grotesk leading-7">
                      Email*
                    </label>
                    <input
                      type="email"
                      placeholder="Email"
                      className="w-full bg-white rounded-[14px] border border-black px-[30px] py-[18px] text-[18px] font-space-grotesk outline-none"
                      required
                    />
                  </div>

                  {/* Message */}
                  <div className="space-y-2">
                    <label className="block text-black text-[16px] font-space-grotesk leading-7">
                      Message*
                    </label>
                    <textarea
                      placeholder="Message"
                      className="w-full bg-white rounded-[14px] border border-black px-[30px] py-[18px] text-[18px] font-space-grotesk outline-none min-h-[190px] resize-none"
                      required
                    />
                  </div>

                  {/* Button */}
                  <button
                    type="submit"
                    className="w-full bg-spartan-dark text-white rounded-[14px] px-[35px] py-[20px] text-[20px] font-space-grotesk"
                  >
                    Send Message
                  </button>
                </form>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer className="relative px-6 pt-12 pb-10 overflow-hidden">
        <div className="max-w-7xl mx-auto">
          <div className="border-t border-[#F8F8FE] pt-10">
            <div className="flex flex-col md:flex-row md:items-start md:justify-between gap-10">
              {/* Left: brand + description + icons */}
              <div className="max-w-sm">
                <div className="text-[#1B1C31] text-[21px] font-bold font-[Manrope]">
                  Spartan Marketplace
                </div>

                <p className="mt-6 text-[#757095] text-[16px] leading-7 font-[Manrope]">
                  A student-verified marketplace for buying and selling tickets.
                </p>

                <div className="mt-6 flex items-center gap-5 text-[#3734A9]">
                  {/* Replace href with your links */}
                  <a href="#" aria-label="LinkedIn" className="w-5 h-5 bg-[#3734A9] rounded-sm" />
                  <a href="#" aria-label="Messenger" className="w-5 h-5 bg-[#3734A9] rounded-sm" />
                  <a href="#" aria-label="Twitter" className="w-5 h-5 bg-[#3734A9] rounded-sm" />
                  <a href="#" aria-label="Infinity" className="w-5 h-5 bg-[#3734A9] rounded-sm" />
                </div>
              </div>

              {/* Right: links */}
              <div className="flex flex-col gap-5 text-[#181433]">
                <Link to="/privacy" className="text-[21px] font-bold font-[Manrope]">
                  Privacy
                </Link>
                <Link to="/terms" className="text-[21px] font-bold font-[Manrope]">
                  Terms
                </Link>
                <Link to="/contact" className="text-[21px] font-bold font-[Manrope]">
                  Contact Us
                </Link>
              </div>
            </div>

            <div className="mt-10 text-center text-[#181433] text-[16px] font-medium font-inter leading-6">
              Copyright @ Spartan Marketplace 2026. All Rights Reserved.
            </div>
          </div>
        </div>
      </footer>
      {/* Green gradient ellipse background (footer) */}
<div
  className="pointer-events-none absolute left-1/2 -translate-x-1/2 -bottom-[420px] w-[2008px] h-[1221px] opacity-60"
  aria-hidden="true"
  style={{ transform: "translateX(-50%) rotate(-1.255deg)" }}
>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width="2008"
    height="1221"
    viewBox="0 0 2008 1221"
    fill="none"
    className="w-full h-full"
  >
    <g opacity="0.6" filter="url(#filter0_f_69_295)">
      <path
        d="M1807.25 592.684C1812.21 819.066 1456.44 1010.47 1012.63 1020.19C568.813 1029.91 205.01 854.271 200.051 627.889C195.092 401.506 550.856 210.106 994.671 200.385C1438.49 190.663 1802.29 366.301 1807.25 592.684Z"
        fill="url(#paint0_linear_69_295)"
        fillOpacity="0.26"
      />
    </g>
    <defs>
      <filter
        id="filter0_f_69_295"
        x="0"
        y="0"
        width="2007.3"
        height="1220.57"
        filterUnits="userSpaceOnUse"
        colorInterpolationFilters="sRGB"
      >
        <feFlood floodOpacity="0" result="BackgroundImageFix" />
        <feBlend mode="normal" in="SourceGraphic" in2="BackgroundImageFix" result="shape" />
        <feGaussianBlur stdDeviation="100" result="effect1_foregroundBlur_69_295" />
      </filter>

      <linearGradient
        id="paint0_linear_69_295"
        x1="1012.85"
        y1="-93.2623"
        x2="1098.72"
        y2="479.67"
        gradientUnits="userSpaceOnUse"
      >
        <stop stopColor="#18453B" />
        <stop offset="1" stopColor="#0FD1BA" stopOpacity="0.29" />
      </linearGradient>
    </defs>
  </svg>
</div>
    </div>
  )
}