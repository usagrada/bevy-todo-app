import React, { VFC } from 'react';
import styled from '@emotion/styled';
import { Link } from 'react-router-dom';
import { Typography } from '@mui/material';

const Header: VFC = () => {
  return (
    <CustomHeader>
      <Link to={'/'}>Home</Link>
      <Link to={'/about'}>About</Link>
      <Link to={'/user'}>User</Link>
      <Typography variant="h6">Title</Typography>
    </CustomHeader>
  );
};

// eslint-disable-next-line @typescript-eslint/no-empty-interface
interface CustomHeaderProps {}

const CustomHeader = styled.header<CustomHeaderProps>`
  /* position: fixed; */
  top: 0;
  left: 0;
  z-index: 30;
  height: 80px;
  width: 80%;
  padding: 10px 10%;
  margin: 0;
`;

export default Header;
