'use client';

import { useState, useEffect } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { useRouter } from 'next/navigation';
import { api } from '@/lib/api';
import { Game } from '@/types';
import { Input } from '@/components/ui/Input';
import { Button } from '@/components/ui/Button';
import { Select } from '@/components/ui/Select';

const sellTicketSchema = z.object({
  game_id: z.string().min(1, 'Please select a game'),
  level: z.string().min(1, 'Level is required'),
  seat_section: z.string().min(1, 'Section is required'),
  seat_row: z.string().min(1, 'Row is required'),
  seat_number: z.string().min(1, 'Seat number is required'),
  price: z.string().refine((val) => !isNaN(Number(val)) && Number(val) > 0, {
    message: 'Price must be a positive number',
  }),
});

type SellTicketFormData = z.infer<typeof sellTicketSchema>;

export function SellTicketForm() {
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [games, setGames] = useState<Game[]>([]);
  const router = useRouter();

  const {
    register,
    handleSubmit,
    formState: { errors },
    reset,
  } = useForm<SellTicketFormData>({
    resolver: zodResolver(sellTicketSchema),
    defaultValues: {
      level: 'STUD',
    },
  });

  useEffect(() => {
    async function fetchGames() {
      try {
        const { games } = await api.getGames();
        setGames(games);
      } catch (err) {
        setError('Failed to load games');
      }
    }
    fetchGames();
  }, []);

  const onSubmit = async (data: SellTicketFormData) => {
    setError(null);
    setSuccess(false);
    setIsLoading(true);

    try {
      await api.createTicket({
        game_id: data.game_id,
        level: data.level,
        seat_section: data.seat_section,
        seat_row: data.seat_row,
        seat_number: data.seat_number,
        price: Math.round(Number(data.price) * 100), // Convert to cents
      });
      setSuccess(true);
      reset();
      setTimeout(() => router.push('/my-listings'), 2000);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create listing');
    } finally {
      setIsLoading(false);
    }
  };

  const gameOptions = [
    { value: '', label: 'Select a game...' },
    ...games.map((game) => ({
      value: game.id,
      label: `${game.name} - ${new Date(game.game_time).toLocaleDateString()}`,
    })),
  ];

  const levelOptions = [
    { value: 'STUD', label: 'Student (STUD)' },
    { value: 'GA', label: 'General Admission (GA)' },
    { value: 'RES', label: 'Reserved (RES)' },
  ];

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      {error && (
        <div className="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg">
          {error}
        </div>
      )}

      {success && (
        <div className="bg-green-50 border border-green-200 text-green-700 px-4 py-3 rounded-lg">
          Ticket listed successfully! Redirecting to your listings...
        </div>
      )}

      <Select
        id="game_id"
        label="Select Game"
        options={gameOptions}
        error={errors.game_id?.message}
        {...register('game_id')}
      />

      <Select
        id="level"
        label="Ticket Level"
        options={levelOptions}
        error={errors.level?.message}
        {...register('level')}
      />

      <div className="grid grid-cols-3 gap-4">
        <Input
          id="seat_section"
          label="Section"
          placeholder="e.g., GEN"
          error={errors.seat_section?.message}
          {...register('seat_section')}
        />

        <Input
          id="seat_row"
          label="Row"
          placeholder="e.g., 128"
          error={errors.seat_row?.message}
          {...register('seat_row')}
        />

        <Input
          id="seat_number"
          label="Seat"
          placeholder="e.g., 28"
          error={errors.seat_number?.message}
          {...register('seat_number')}
        />
      </div>

      <Input
        id="price"
        type="number"
        step="0.01"
        min="0"
        label="Price ($)"
        placeholder="e.g., 150.00"
        error={errors.price?.message}
        {...register('price')}
      />

      <Button type="submit" className="w-full" disabled={isLoading}>
        {isLoading ? 'Creating listing...' : 'List Ticket'}
      </Button>

      <div className="bg-yellow-50 border border-yellow-200 text-yellow-800 px-4 py-3 rounded-lg text-sm">
        <strong>Important:</strong> After listing, you must transfer your ticket
        to our Paciolan custodial account within 24 hours for verification.
      </div>
    </form>
  );
}
