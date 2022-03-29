import React, { lazy, Suspense, VFC } from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { RecoilRoot } from 'recoil';
import CustomThemeProvider from '../utils/provider/themeProvider';
import TaskPage from './pages/TaskPage';

const TopPage = lazy(() => import('./pages/TopPage'));
const NotFound = lazy(() => import('./pages/NotFound'));
const AboutPage = lazy(() => import('./pages/AboutPage'));
const UserPage = lazy(() => import('./pages/UserPage'));

const App: VFC = () => {
  return (
    <RecoilRoot>
      <CustomThemeProvider>
        <Suspense fallback={() => 'hello'}>
          {/* <Router basename={process.env.PUBLIC_URL}> */}
            {/* <Routes> */}
            {/* <Route path="/" element={<TopPage />} /> */}
            {/* <Route path="/about" element={<AboutPage />} /> */}
            {/* <Route path="/user" element={<UserPage />} /> */}
            {/* <Route element={<NotFound />} /> */}
            {/* </Routes> */}
          {/* </Router> */}
          <TaskPage />
        </Suspense>
      </CustomThemeProvider>
    </RecoilRoot>
  );
};

export default App;
