import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { AuthGuard } from './guards/auth.guard';

const routes: Routes = [
  {
    path: 'auth',
    canActivate: [AuthGuard],
    component: AuthGuard,
    data: {
      //todo: make this external
      externalUrl: 'http://localhost:8888/api/v1/auth',
    },
  },
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule],
})
export class AppRoutingModule {}
