import { Container } from '@mui/material';
import React, { VFC } from 'react';
import Footer from '../layouts/Footer';
import Header from '../layouts/Header';

const TopPage: VFC = () => {
  return (
    <>
      <Header />
      <Container>
        <main>
          <h1>Top Page</h1>
          <div>ここがmain コンテンツです</div>
        </main>
      </Container>
      <Footer />
    </>
  );
};

export default TopPage;
