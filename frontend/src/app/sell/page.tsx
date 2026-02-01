'use client';

import { Navbar } from '@/components/layout/Navbar';
import { Footer } from '@/components/layout/Footer';
import { Card, CardContent, CardHeader } from '@/components/ui/Card';
import { SellTicketForm } from '@/components/forms/SellTicketForm';
import { AuthGuard } from '@/components/layout/AuthGuard';

export default function SellPage() {
  return (
    <AuthGuard>
      <div className="min-h-screen flex flex-col">
        <Navbar />
        <main className="flex-1 px-6 py-12">
          <div className="max-w-xl mx-auto">
            <Card>
              <CardHeader>
                <h1 className="text-2xl font-bold text-spartan-black">
                  List Your Ticket
                </h1>
                <p className="text-gray-600 mt-1">
                  Fill in your ticket details to create a listing
                </p>
              </CardHeader>
              <CardContent>
                <SellTicketForm />
              </CardContent>
            </Card>
          </div>
        </main>
        <Footer />
      </div>
    </AuthGuard>
  );
}
