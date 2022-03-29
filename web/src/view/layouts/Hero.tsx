import React, { VFC } from 'react';
import styled from '@emotion/styled';
import { Toolbar, Typography } from '@mui/material';

const Hero: VFC = () => {
  return (
    <CustomHeader>
      <CustomToolbar>
        <Typography variant="h6">Title</Typography>
      </CustomToolbar>
    </CustomHeader>
  );
};

const CustomHeader = styled.div`
  position: static;
  /* box-sizing: border-box; */
  height: 80vh;
  width: 80%;
  padding: 10vh 10%;
  margin: 0;
`;

const CustomToolbar = styled(Toolbar)`
  && {
    height: 80%;
    width: 80%;
    margin: 0% 0%;
    padding: 10% 10%;
    display: flex;
    align-items: center;
    justify-content: flex-start;
    flex-wrap: wrap;
  }
`;

export default Hero;
