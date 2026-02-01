'use client';

import { useState, useEffect } from 'react';
import { Navbar } from '@/components/layout/Navbar';
import { Footer } from '@/components/layout/Footer';
import { Card, CardContent } from '@/components/ui/Card';
import { api } from '@/lib/api';
import { Game } from '@/types';
import { formatDateTime } from '@/lib/utils';

export default function GamesPage() {
  const [games, setGames] = useState<Game[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function fetchGames() {
      try {
        const { games } = await api.getGames();
        setGames(games);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to load games');
      } finally {
        setIsLoading(false);
      }
    }
    fetchGames();
  }, []);

  const sportEmoji = (sport: string) => {
    switch (sport) {
      case 'Football':
        return '🏈';
      case 'Basketball':
        return '🏀';
      case 'Hockey':
        return '🏒';
      default:
        return '🎟️';
    }
  };

  return (
    <div className="min-h-screen flex flex-col">
      <Navbar />
      <main className="flex-1 px-6 py-12">
        <div className="max-w-4xl mx-auto">
          <h1 className="text-3xl font-bold text-spartan-black mb-8">
            Upcoming Games
          </h1>

          {isLoading ? (
            <div className="flex justify-center py-12">
              <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-spartan-green"></div>
            </div>
          ) : error ? (
            <div className="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg">
              {error}
            </div>
          ) : games.length === 0 ? (
            <div className="text-center py-12 text-gray-500">
              No upcoming games at the moment.
            </div>
          ) : (
            <div className="space-y-4">
              {games.map((game) => (
                <Card key={game.id} className="hover:shadow-lg transition-shadow">
                  <CardContent className="p-6">
                    <div className="flex items-center gap-4">
                      <div className="text-4xl">{sportEmoji(game.sport_type)}</div>
                      <div className="flex-1">
                        <h2 className="text-xl font-semibold text-spartan-black">
                          {game.name}
                        </h2>
                        <p className="text-gray-600">
                          {formatDateTime(game.game_time)}
                        </p>
                        <p className="text-sm text-gray-500 mt-1">
                          Trading closes: {formatDateTime(game.cutoff_time)}
                        </p>
                      </div>
                      <div className="text-right">
                        <span className="inline-block px-3 py-1 bg-spartan-green/10 text-spartan-green rounded-full text-sm font-medium">
                          {game.sport_type}
                        </span>
                      </div>
                    </div>
                  </CardContent>
                </Card>
              ))}
            </div>
          )}
        </div>
      </main>
      <Footer />
    </div>
  );
}
