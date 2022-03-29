import React, { VFC } from 'react';
import Header from '../layouts/Header';
import Footer from '../layouts/Footer';
import Container from '../layouts/Container';
const AboutPage: VFC = () => {
  return (
    <>
      <Header />
      <Container>
        <main>
          <div>About Page</div>
        </main>
      </Container>
      <Footer />
    </>
  );
};

export default AboutPage;
