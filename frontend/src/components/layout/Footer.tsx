export function Footer() {
  return (
    <footer className="bg-spartan-black text-spartan-white py-8 px-6">
      <div className="max-w-7xl mx-auto">
        <div className="flex flex-col md:flex-row justify-between items-center gap-4">
          <div>
            <h3 className="text-xl font-bold text-spartan-green">MSU Ticket Marketplace</h3>
            <p className="text-sm text-gray-400 mt-1">
              Buy and sell MSU student sports tickets safely
            </p>
          </div>
          <div className="text-sm text-gray-400">
            &copy; {new Date().getFullYear()} MSU Ticket Marketplace. For MSU students only.
          </div>
        </div>
      </div>
    </footer>
  );
}
