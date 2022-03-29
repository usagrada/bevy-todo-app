/**
 * @jest-environment jsdom
 */
import React from 'react';
import { render, screen } from '@testing-library/react';
import Footer from '../view/layouts/Footer';

describe('snapshot test', () => {
  test('sample test', () => {
    const a = render(<Footer />);
    expect(a).toMatchSnapshot();
  });

  it('test', () => {
    render(<Footer />);
    screen.debug();
    expect(screen.getByText('Footer')).toBeInTheDocument();
  });
});
