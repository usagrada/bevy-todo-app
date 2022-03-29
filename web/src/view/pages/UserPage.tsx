import { Container } from '@mui/material';
import React, { VFC } from 'react';
import Footer from '../layouts/Footer';
import Header from '../layouts/Header';

const UserPage: VFC = () => {
  return (
    <>
      <Header />
      <Container>
        <main>
          <h1>User Page</h1>
          <div>ここがmain コンテンツです</div>
        </main>
      </Container>
      <Footer />
    </>
  );
};

export default UserPage;
